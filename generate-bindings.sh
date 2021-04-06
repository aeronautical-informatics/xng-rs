#!/usr/bin/env nix-shell
#!nix-shell -i bash --pure --packages rust-bindgen rustfmt

set -e
cd "$(dirname "$0")"
export CPATH=../include
bindgen wrapper.h --ctypes-prefix cty --use-core --output src/raw_bindings.rs
