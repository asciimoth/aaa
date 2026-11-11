#!/bin/sh
# The short delays keep each worker visible in the progress demo.
set -eu

zpg --report 0 2 --name "frame $AAA_FRAME: prepare"
sleep "${AAA_DEMO_DELAY:-0.15}"
zpg --report 1 2 --name "frame $AAA_FRAME: overlay"
# This script owns the reporting pipe. Its pipeline must not also write reports.
ZIG_PROGRESS= ./examples/overlay_filter.sh
sleep "${AAA_DEMO_DELAY:-0.15}"
zpg --report 2 2 --name "frame $AAA_FRAME: done"
