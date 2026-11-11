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
