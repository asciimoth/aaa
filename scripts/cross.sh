#!/usr/bin/env bash
set -euo pipefail

# cross needs the rustup proxies even when Nix puts its own Cargo first.
export PATH="$(dirname -- "$(command -v rustup)"):$PATH"
exec cross "$@"
