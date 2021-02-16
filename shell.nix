let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  pkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  ske-server = import ../. { inherit pkgs; };
in with pkgs; stdenv.mkDerivation {
  name = "xng-rs";

  nativeBuildInputs = [ llvmPackages.clang-unwrapped.lib ];
  buildInputs = [
    latest.rustChannels.beta.rust
    ske-server
 ];

  shellHook = ''
    export NIX_ENFORCE_PURITY=0;
    export LIBCLANG_PATH="${llvmPackages.clang-unwrapped.lib}/lib";
    export CPATH=${ske-server}/include:
    exec zsh
  '';
}
