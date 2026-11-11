#!/usr/bin/env bash
set -euo pipefail

fail() { printf 'release: %s\n' "$*" >&2; exit 1; }
[[ $# == 2 && ( $1 == check || $1 == publish ) ]] ||
    fail 'Usage: release.sh check|publish TAG'
mode=$1
tag=$2
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

for tool in git just goreleaser cargo rustup cross docker python3 nix; do
    command -v "$tool" >/dev/null || fail "Missing command: $tool (enter nix develop)."
done
version=$(python3 -c 'import tomllib; print(tomllib.load(open("Cargo.toml", "rb"))["package"]["version"])')
if [[ $mode == check && -z $tag ]]; then tag=$version; fi
[[ $tag =~ ^v?(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)$ ]] ||
    fail 'Tag must be x.y.z or vx.y.z.'
[[ ${tag#v} == "$version" ]] || fail "Tag $tag does not match Cargo.toml version $version."
python3 - <<'PY'
import tomllib
with open("Cargo.toml", "rb") as f:
    package = tomllib.load(f)["package"]
with open("Cargo.lock", "rb") as f:
    locked = tomllib.load(f)["package"]
if not any(p["name"] == package["name"] and p["version"] == package["version"] and "source" not in p for p in locked):
    raise SystemExit("release: Update Cargo.lock to match Cargo.toml.")
PY

goreleaser --version | python3 -c '
import re, sys
version = re.search(r"GitVersion:\s+v?(\d+)\.(\d+)\.", sys.stdin.read())
if version is None or tuple(map(int, version.groups())) < (2, 19):
    raise SystemExit("release: GoReleaser 2.19 or newer is required (enter nix develop).")
'
goreleaser check
docker info >/dev/null

created_tag=false
pushed_tag=false
cleanup() {
    result=$?
    if [[ $created_tag == true && $pushed_tag == false ]] &&
        [[ $(git rev-parse "refs/tags/$tag^{commit}") == "$head" ]]; then
        git tag -d "$tag" >/dev/null
    fi
    exit "$result"
}
trap cleanup EXIT

if [[ $mode == publish ]]; then
    for tool in gh gpg curl; do command -v "$tool" >/dev/null || fail "Missing command: $tool"; done
    [[ -z $(git status --porcelain) ]] || fail 'Commit all changes before releasing.'
    [[ $(git rev-parse --is-shallow-repository) == false ]] || fail 'Fetch the full Git history and tags.'
    head=$(git rev-parse HEAD)
    if git rev-parse -q --verify "refs/tags/$tag" >/dev/null; then fail "Local tag $tag already exists."; fi
    origin=$(git remote get-url --push --all origin)
    [[ $origin != *$'\n'* ]] || fail 'origin must have one push URL.'
    case $origin in
        https://github.com/*) github_repo=${origin#https://github.com/} ;;
        git@github.com:*) github_repo=${origin#git@github.com:} ;;
        ssh://git@github.com/*) github_repo=${origin#ssh://git@github.com/} ;;
        *) fail 'origin must point to a github.com repository.' ;;
    esac
    github_repo=${github_repo%.git}
    [[ $github_repo =~ ^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$ ]] || fail 'Invalid GitHub origin.'
    remote_tags=$(git ls-remote --tags "$origin" "refs/tags/$tag")
    [[ -z $remote_tags ]] || fail "Remote tag $tag already exists."
    export GH_HOST=github.com
    export GITHUB_TOKEN=${GITHUB_TOKEN:-${GH_TOKEN:-}}
    if [[ -z $GITHUB_TOKEN ]]; then GITHUB_TOKEN=$(gh auth token --hostname github.com); fi
    [[ -n $GITHUB_TOKEN ]] || fail 'Set GITHUB_TOKEN or GH_TOKEN, or use gh auth login.'
    export GH_TOKEN=$GITHUB_TOKEN
    [[ $(gh api "repos/$github_repo" --jq '.permissions.push') == true ]] || fail 'GitHub token needs repository write access.'
    if gh release view "$tag" --repo "github.com/$github_repo" >/dev/null 2>&1; then
        fail "GitHub release $tag already exists."
    fi
    export GPG_KEY_ID=${GPG_KEY_ID:-C5C84658CCFD7E8E71DEE933AF3AE54FC3A35C9F}
    gpg --list-secret-keys "$GPG_KEY_ID" >/dev/null
    [[ -n ${CARGO_REGISTRY_TOKEN:-${CARGO_REGISTRIES_CRATES_IO_TOKEN:-}} ]] ||
        fail 'Set CARGO_REGISTRY_TOKEN or CARGO_REGISTRIES_CRATES_IO_TOKEN for crates.io.'
    crate_status=$(curl --silent --show-error --output /dev/null --write-out '%{http_code}' \
        "https://crates.io/api/v1/crates/aaa/$version")
    case $crate_status in
        404) ;;
        200) fail "aaa $version already exists on crates.io. Set a new version before releasing." ;;
        *) fail "Cannot check crates.io version (HTTP $crate_status)." ;;
    esac
    if [[ -n ${GHREPO_REPO_DIR:-} ]]; then
        command -v ghrepo >/dev/null || fail 'ghrepo is required when GHREPO_REPO_DIR is set.'
        [[ -d $GHREPO_REPO_DIR && -w $GHREPO_REPO_DIR && -r $GHREPO_REPO_DIR/ghrepo.yaml ]] ||
            fail 'GHREPO_REPO_DIR must be a writable, initialized ghrepo repository.'
        ghrepo check --dry-run
    fi
fi

just check
cargo publish --dry-run --locked --allow-dirty --registry crates-io
just check-nix

if [[ $mode == check ]]; then
    GORELEASER_CURRENT_TAG=$tag goreleaser release --snapshot --clean --skip=sign,publish --timeout=60m
    python3 scripts/check_artifacts.py
    printf 'Release check passed. Unsigned artifacts: dist/; crate: target/package/\n'
    exit 0
fi

[[ -z $(git status --porcelain) && $(git rev-parse HEAD) == "$head" ]] ||
    fail 'Source changed during checks. Commit changes before releasing.'
git -c tag.gpgSign=false tag -a "$tag" "$head" -m "Release $tag"
created_tag=true
GORELEASER_CURRENT_TAG=$tag goreleaser release --clean --skip=publish --timeout=60m
python3 scripts/check_artifacts.py --signed

assets=(dist/checksums.txt dist/checksums.txt.sig)
urls=()
while read -r checksum name; do
    assets+=("dist/$name" "dist/$name.sig")
    case $name in
        *.deb|*.rpm) urls+=("https://github.com/$github_repo/releases/download/$tag/$name") ;;
    esac
done < dist/checksums.txt
[[ -z $(git status --porcelain) && $(git rev-parse HEAD) == "$head" ]] ||
    fail 'Source changed during packaging; cannot publish.'
[[ $(git rev-parse "refs/tags/$tag^{commit}") == "$head" ]] || fail 'Release tag changed during packaging.'

git push origin "refs/tags/$tag"
pushed_tag=true
gh release create "$tag" "${assets[@]}" --repo "github.com/$github_repo" --verify-tag --title "$tag" --generate-notes
cargo publish --locked --registry crates-io
if [[ -n ${GHREPO_REPO_DIR:-} ]]; then
    ghrepo add "${urls[@]}"
    ghrepo publish --message "chore(repo): publish aaa $tag"
fi
printf 'Published https://github.com/%s/releases/tag/%s and aaa %s on crates.io.\n' "$github_repo" "$tag" "$version"
