#!/bin/sh
set -eu

name=${1:-aaa}

mkdir -p ./target/completions/bash
cargo run --locked -- completions bash > "./target/completions/bash/$name"

mkdir -p ./target/completions/fish
cargo run --locked -- completions fish > "./target/completions/fish/$name.fish"

mkdir -p ./target/completions/zsh
cargo run --locked -- completions zsh > "./target/completions/zsh/_$name"

mkdir -p ./target/completions/nu
cargo run --locked -- completions nush > "./target/completions/nu/$name.nu"

