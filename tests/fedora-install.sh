#!/bin/sh
# Run from a Docker host. Pass an optional RPM path to test a local build.
set -eu

if [ "$#" -gt 1 ]; then
    printf 'Usage: %s [path-to-rpm]\n' "$0" >&2
    exit 2
fi

package=${1:-}
set -- --rm -i --platform "${PLATFORM:-linux/amd64}"
if [ -n "$package" ]; then
    if [ ! -f "$package" ]; then
        printf 'RPM not found: %s\n' "$package" >&2
        exit 2
    fi
    package_dir=$(CDPATH= cd -- "$(dirname -- "$package")" && pwd)
    package="$package_dir/$(basename -- "$package")"
    set -- "$@" --mount "type=bind,src=$package,dst=/tmp/aaa.rpm,readonly" \
        --env AAA_TEST_PACKAGE=/tmp/aaa.rpm
fi

exec docker run "$@" fedora:44 sh -se <<'CONTAINER'
dnf -y --setopt=install_weak_deps=False install \
    "https://mirrors.rpmfusion.org/free/fedora/rpmfusion-free-release-$(rpm -E %fedora).noarch.rpm"
curl -fsSLo /etc/yum.repos.d/ghrepo.repo \
    https://repo.moth.contact/rpm.repo
dnf -y --setopt=install_weak_deps=False install --allowerasing \
    "${AAA_TEST_PACKAGE:-aaa}"
rpm -q aaa ffmpeg
aaa --help > /dev/null

# Install the WebP decoder only after aaa has resolved its own dependencies.
dnf -y --setopt=install_weak_deps=False install libwebp-tools
cat > /tmp/input.3a <<'ART'
@3a
colors no
loop no

@body
AA
AA

BB
BB
ART

aaa convert /tmp/input.3a to-mp4 < /dev/null > /tmp/output.mp4
codec=$(ffprobe -v error -select_streams v:0 -show_entries stream=codec_name \
    -of default=noprint_wrappers=1:nokey=1 /tmp/output.mp4)
test "$codec" = h264
ffmpeg -nostdin -v error -xerror -i /tmp/output.mp4 -f null -

aaa convert /tmp/input.3a to-webp < /dev/null > /tmp/output.webp
webpmux -info /tmp/output.webp
for frame in 1 2; do
    webpmux -get frame "$frame" /tmp/output.webp -o /tmp/frame.webp
    dwebp /tmp/frame.webp -o /tmp/frame.png
done
printf 'Fedora %s: installation, MP4, and WebP checks passed.\n' "$(uname -m)"
CONTAINER
