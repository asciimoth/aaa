"""Test release failures and publication order without remote changes."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[1]
MOCK = r'''#!/usr/bin/env python3
import hashlib, json, os, pathlib, sys
name = pathlib.Path(sys.argv[0]).name
args = sys.argv[1:]
with open(os.environ["CALL_LOG"], "a") as log:
    log.write(json.dumps([name, *args]) + "\n")
if name == "git":
    if args[0] == "status" and os.environ.get("DIRTY"): print(" M Cargo.toml")
    elif args[:3] == ["rev-parse", "-q", "--verify"]:
        sys.exit(0 if os.environ.get("EXISTING_TAG") else 1)
    elif args[:2] == ["rev-parse", "--is-shallow-repository"]: print("false")
    elif args[:2] == ["remote", "get-url"]: print("git@github.com:example/aaa.git")
    elif args[0] == "rev-parse": print("abc123")
elif name == "gh":
    if args[0] == "api": print("true")
    elif args[:2] == ["release", "view"]: sys.exit(1)
elif name == "curl":
    version = "1.2.3" if os.environ.get("EXISTING_CRATE") else "1.2.2"
    print(json.dumps({"vers": version}))
elif name == "goreleaser" and args == ["--version"]:
    print("GitVersion: 2.19.0")
elif name == "gpg" and "--verify" in args and os.environ.get("FAIL_SIGNATURE"):
    sys.exit(1)
elif name == "cargo" and "--dry-run" in args and os.environ.get("FAIL_CRATE"):
    sys.exit(1)
elif name == "just" and args == ["check-nix"] and os.environ.get("FAIL_NIX"):
    sys.exit(1)
elif name == "goreleaser" and args[0] == "release":
    if os.environ.get("FAIL_BUILD"): sys.exit(1)
    dist = pathlib.Path("dist"); dist.mkdir(exist_ok=True)
    artifacts = []
    targets = [("linux", a) for a in ("amd64", "386", "arm64", "riscv64")]
    targets.append(("freebsd", "amd64"))
    for goos, arch in targets:
        base = f"aaa_1.2.3_{goos}_{arch}"
        artifacts.append(dict(name=base + ".tar.gz", type="Archive", goos=goos, goarch=arch))
        if goos == "linux":
            for fmt in ("deb", "rpm", "archlinux"):
                if arch == "riscv64" and fmt == "archlinux": continue
                artifacts.append(dict(name=base + "." + fmt, type="Linux Package",
                                      goos=goos, goarch=arch, extra={"Format": fmt}))
    lines = []
    for artifact in artifacts:
        filename = artifact["name"]
        (dist / filename).write_bytes(b"package")
        lines.append(hashlib.sha256(b"package").hexdigest() + "  " + filename)
        if "--snapshot" not in args:
            (dist / (filename + ".sig")).write_bytes(b"signature")
    (dist / "checksums.txt").write_text("\n".join(lines) + "\n")
    (dist / "artifacts.json").write_text(json.dumps(artifacts))
    if "--snapshot" not in args:
        (dist / "checksums.txt.sig").write_bytes(b"signature")
    if os.environ.get("BAD_CHECKSUM"):
        (dist / artifacts[0]["name"]).write_bytes(b"damaged")
'''


class ReleaseTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        shutil.copytree(ROOT / "scripts", self.root / "scripts")
        (self.root / "Cargo.toml").write_text('[package]\nname = "aaa"\nversion = "1.2.3"\n')
        (self.root / "Cargo.lock").write_text('[[package]]\nname = "aaa"\nversion = "1.2.3"\n')
        mock_bin = self.root / "bin"
        mock_bin.mkdir()
        mock = mock_bin / "mock"
        mock.write_text(MOCK)
        mock.chmod(0o755)
        for name in ("git", "just", "goreleaser", "cargo", "rustup", "cross", "docker",
                     "nix", "gh", "gpg", "ghrepo", "curl"):
            (mock_bin / name).symlink_to(mock)
        self.log = self.root / "calls.jsonl"
        self.env = dict(os.environ, PATH=f"{mock_bin}:{os.environ['PATH']}",
                        CALL_LOG=str(self.log), GITHUB_TOKEN="test-token", CARGO_REGISTRY_TOKEN="test-token")
        for name in ("GHREPO_REPO_DIR", "DIRTY", "EXISTING_TAG", "EXISTING_CRATE",
                     "FAIL_CRATE", "FAIL_NIX", "FAIL_BUILD", "BAD_CHECKSUM", "FAIL_SIGNATURE"):
            self.env.pop(name, None)

    def run_release(self, mode="publish", tag="1.2.3"):
        return subprocess.run(["bash", "scripts/release.sh", mode, tag], cwd=self.root,
                              env=self.env, capture_output=True, text=True, timeout=20)

    def calls(self):
        return [json.loads(line) for line in self.log.read_text().splitlines()] if self.log.exists() else []

    def assert_no_publication(self):
        for call in self.calls():
            self.assertNotEqual(call[:2], ["git", "push"])
            self.assertNotEqual(call[:3], ["gh", "release", "create"])
            self.assertNotEqual(call[:2], ["ghrepo", "add"])
            self.assertNotEqual(call[:2], ["ghrepo", "publish"])
            if call[:2] == ["cargo", "publish"]:
                self.assertIn("--dry-run", call)

    def test_dry_run_needs_no_credentials_and_does_not_sign_or_publish(self):
        for key in ("GITHUB_TOKEN", "GH_TOKEN", "CARGO_REGISTRY_TOKEN", "CARGO_REGISTRIES_CRATES_IO_TOKEN"):
            self.env.pop(key, None)
        self.env["GHREPO_REPO_DIR"] = "/unused-in-dry-run"
        result = self.run_release("check", "")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assert_no_publication()
        calls = self.calls()
        self.assertIn(["just", "check-nix"], calls)
        self.assertTrue(any(c[:2] == ["goreleaser", "release"] and "--skip=sign,publish" in c for c in calls))
        self.assertFalse(any(c[0] in ("gh", "gpg", "ghrepo", "curl") for c in calls))
        self.assertFalse(any("tag" in c for c in calls if c[0] == "git"))

    def test_version_mismatch_stops_before_builds(self):
        result = self.run_release(tag="9.9.9")
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.calls(), [])

    def test_failure_stops_publication(self):
        for flag in ("DIRTY", "EXISTING_TAG", "EXISTING_CRATE", "FAIL_CRATE", "FAIL_NIX", "FAIL_BUILD", "BAD_CHECKSUM", "FAIL_SIGNATURE"):
            with self.subTest(flag=flag):
                self.log.unlink(missing_ok=True)
                self.env[flag] = "1"
                result = self.run_release()
                self.assertNotEqual(result.returncode, 0, result.stdout)
                self.assert_no_publication()
                if flag in ("FAIL_BUILD", "BAD_CHECKSUM", "FAIL_SIGNATURE"):
                    self.assertIn(["git", "tag", "-d", "1.2.3"], self.calls())
                self.env.pop(flag)

    def test_publish_order_and_optional_repository(self):
        repo = self.root / "repository"
        repo.mkdir()
        (repo / "ghrepo.yaml").touch()
        self.env["GHREPO_REPO_DIR"] = str(repo)
        result = self.run_release(tag="v1.2.3")
        self.assertEqual(result.returncode, 0, result.stderr)
        calls = self.calls()
        push = next(i for i, c in enumerate(calls) if c[:2] == ["git", "push"])
        publish = next(i for i, c in enumerate(calls) if c[:3] == ["gh", "release", "create"])
        crate = next(i for i, c in enumerate(calls) if c[:2] == ["cargo", "publish"] and "--dry-run" not in c)
        add = next(i for i, c in enumerate(calls) if c[:2] == ["ghrepo", "add"])
        repository = next(i for i, c in enumerate(calls) if c[:2] == ["ghrepo", "publish"])
        self.assertLess(push, publish)
        self.assertLess(publish, crate)
        self.assertLess(crate, add)
        self.assertLess(add, repository)
        verifications = [i for i, c in enumerate(calls) if c[:2] == ["gpg", "--verify"]]
        self.assertEqual(len(verifications), 17)
        self.assertLess(max(verifications), push)
        self.assertEqual(len(calls[add][2:]), 8)
        for url in calls[add][2:]:
            self.assertTrue(url.startswith("https://github.com/example/aaa/releases/download/v1.2.3/"))
            self.assertTrue(url.endswith((".deb", ".rpm")))

    def test_release_without_repository(self):
        self.env.pop("CARGO_REGISTRY_TOKEN", None)
        self.env.pop("CARGO_REGISTRIES_CRATES_IO_TOKEN", None)
        result = self.run_release()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(any(c[0] == "ghrepo" for c in self.calls()))
        self.assertTrue(any(c[:2] == ["cargo", "publish"] and "--dry-run" not in c
                            for c in self.calls()))


if __name__ == "__main__":
    unittest.main()
