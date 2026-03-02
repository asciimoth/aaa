#!/bin/sh

mkdir -p ./target/completions/bash
cargo run -- completions bash > "./target/completions/bash/$1"

mkdir -p ./target/completions/fish
cargo run -- completions fish > "./target/completions/fish/$1.fish"

mkdir -p ./target/completions/zsh
cargo run -- completions zsh > "./target/completions/zsh/_$1"

