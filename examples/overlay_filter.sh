#!/usr/bin/env bash
set -euo pipefail
# Usage: aaa edit distro_nixos_big filter ansi ./examples/overlay_filter.sh | aaa play

# Only the final pipeline process reports to the inherited progress pipe.
# Concurrent reporters must have separate pipes, as aaa filter provides.
ZIG_PROGRESS= aaa from-text | ZIG_PROGRESS= aaa edit print-ansi 0 0 0 "[30m[43m$AAA_FRAME[34m[49m" | aaa preview

