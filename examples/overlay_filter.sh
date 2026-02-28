#!/bin/sh
# Usage: aaa edit distro_nixos_big filter ansi ./examples/overlay_filter.sh | aaa play

aaa from-text | aaa edit print-ansi 0 0 0 "[30m[43m$AAA_FRAME[34m[49m" | aaa preview

