{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
      in
      rec {
        packages.xng-rs = with pkgs; naersk-lib.buildPackage {
          pname = "xng-rs";
          root = ./.;
          doCheck = true;
          doDoc = true;
          copyLibs = true;
          doDocFail = true;
          copyTarget = true;
        };
        defaultPackage = packages.xng-rs;

        devShell = pkgs.mkShell {
          # ensure that the env var C_INCLUDE_PATH points to a folder with the
          # XNG header files!
          LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
          nativeBuildInputs = with pkgs; [ rustc cargo llvmPackages.llvm ];
        };

        hydraJobs = packages.my-project;
      });
}
