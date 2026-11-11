"""Check the complete release matrix, checksums, and optional GPG signatures."""

import argparse
import hashlib
import json
from pathlib import Path
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--signed", action="store_true")
    args = parser.parse_args()
    dist = Path("dist")
    artifacts = json.loads((dist / "artifacts.json").read_text())
    archives = [a for a in artifacts if a["type"] == "Archive"]
    packages = [a for a in artifacts if a["type"] == "Linux Package"]
    targets = {("linux", arch) for arch in ("amd64", "386", "arm64", "riscv64")}
    targets.add(("freebsd", "amd64"))
    if len(archives) != 5 or {(a["goos"], a["goarch"]) for a in archives} != targets:
        raise SystemExit("expected one archive for each of the five release targets")
    expected_packages = {
        (arch, fmt)
        for arch in ("amd64", "386", "arm64", "riscv64")
        for fmt in ("deb", "rpm", "archlinux")
        if (arch, fmt) != ("riscv64", "archlinux")
    }
    if len(packages) != len(expected_packages) or {
        (a["goarch"], a["extra"]["Format"]) for a in packages
    } != expected_packages:
        raise SystemExit("expected four DEB, four RPM, and three Arch packages")
    expected_names = {a["name"] for a in archives + packages}
    checked = set()
    for line in (dist / "checksums.txt").read_text().splitlines():
        expected, name = line.split("  ", 1)
        if name not in expected_names or name in checked or Path(name).name != name:
            raise SystemExit(f"unexpected checksum entry: {name}")
        if hashlib.sha256((dist / name).read_bytes()).hexdigest() != expected:
            raise SystemExit(f"checksum mismatch: {name}")
        checked.add(name)
    if checked != expected_names:
        raise SystemExit("checksum file does not cover all release artifacts")
    if args.signed:
        for name in sorted(checked | {"checksums.txt"}):
            subprocess.run(
                ["gpg", "--verify", str(dist / f"{name}.sig"), str(dist / name)],
                check=True,
            )
    print(f"Verified {len(archives)} archives and {len(packages)} packages.")


if __name__ == "__main__":
    main()
