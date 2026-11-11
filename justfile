set dotenv-load := true

# List the development commands.
default:
    @just --list

# Build the CLI.
[positional-arguments]
build *args:
    cargo build --locked "$@"

# Build and run the CLI.
[positional-arguments]
run *args:
    cargo run --locked -- "$@"

# Filter and play the NixOS animation with progress on stderr.
[positional-arguments]
demo-edit jobs="4": build
    #!/usr/bin/env bash
    set -euo pipefail
    export PATH="$PWD/target/debug:$PATH"
    zpg aaa edit distro_nixos_big filter --jobs "$1" ansi ./examples/overlay_filter.sh | aaa play --loop false

# Show parallel filter scripts with their own progress reports (try jobs=1 too).
[positional-arguments]
demo-filter jobs="4" output="target/demo-filter.3a": build
    #!/usr/bin/env bash
    set -euo pipefail
    export PATH="$PWD/target/debug:$PATH"
    zpg aaa edit distro_nixos_big filter --jobs "$1" ansi ./examples/progress_filter.sh > "$2"

# Render frames in parallel, then encode them as a GIF in frame order.
[positional-arguments]
demo-gif jobs="4" output="target/demo.gif": build
    zpg target/debug/aaa convert distro_nixos_big to-gif --jobs "$1" --font-size 12 > "$2"

# Render and save frames in parallel, then run the FFmpeg child process.
[positional-arguments]
demo-mp4 jobs="4" output="target/demo.mp4": build
    zpg target/debug/aaa convert distro_nixos_big to-mp4 --jobs "$1" --font-size 12 > "$2"

# Render frames in parallel and encode a WebP animation with FFmpeg.
[positional-arguments]
demo-webp jobs="4" output="target/demo.webp": build
    zpg target/debug/aaa convert distro_nixos_big to-webp --jobs "$1" --font-size 12 > "$2"

# Show row progress and encoding for one PNG on a single thread.
[positional-arguments]
demo-png output="target/demo.png": build
    zpg target/debug/aaa convert distro_nixos_big to-png --font-size 64 > "$1"

# Validate progress packets, nested child reports, and command output with zpg.
test-progress:
    cargo test --locked --test progress -- --ignored

# Run unit, integration, and documentation tests.
[positional-arguments]
test *args:
    cargo test --workspace --all-features --locked "$@"

# Run all local checks without changing files.
check: fmt-check lint test

# Check Rust, Nix, and spelling.
lint: typos
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
    deadnix --fail flake.nix
    statix check flake.nix

# Check spelling.
typos:
    typos

# Format Rust, Nix, and Just files.
fmt:
    cargo fmt --all
    nixfmt flake.nix
    just --unstable --fmt

# Check formatting without changing files.
fmt-check:
    cargo fmt --all -- --check
    nixfmt --check flake.nix
    just --unstable --fmt --check

# Run all pre-commit hooks. Enter nix develop first.
hooks:
    pre-commit run --all-files

# Run the Fedora package test (requires Docker and network access).
[positional-arguments]
test-package *args:
    tests/fedora-install.sh "$@"

# Remove Cargo build output.
clean:
    cargo clean
