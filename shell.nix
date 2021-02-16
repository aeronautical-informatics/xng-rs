{ pkgs ? import <nixpkgs> { }}:

with pkgs;

let 
  ske-server = import ../. { inherit pkgs; };
in stdenv.mkDerivation {
  name = "xng-rs";

  nativeBuildInputs = [ llvmPackages.clang-unwrapped.lib ];
  buildInputs = [ ske-server ];

  shellHook = ''
    export NIX_ENFORCE_PURITY=0;
    export LIBCLANG_PATH="${llvmPackages.clang-unwrapped.lib}/lib";
    exec zsh
  '';
}
