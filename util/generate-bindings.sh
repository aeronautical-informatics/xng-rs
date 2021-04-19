#!/usr/bin/env nix-shell
#!nix-shell -i bash --pure --packages rust-bindgen.override{clang=clang_multi;} rustfmt

set -e
cd "$(dirname "$0")"
export CPATH=./include

gen_bindings() {
	local target="$1"
	local suffix="$2"
	bindgen wrapper.h --ctypes-prefix cty --use-core \
		--whitelist-var '(x|X).*' \
		--whitelist-type '(x|X).*' \
		--whitelist-function '(x|X).*' \
		--output ../src/bindings/$suffix.rs -- \
		-target $target
}

gen_bindings i386-unknown-none-linux 32
gen_bindings x86_64-unknown-none-linux 64
