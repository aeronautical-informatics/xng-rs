{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk}:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
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
        nativeBuildInputs = with pkgs; [ rustc cargo ];
      };

      hydraJobs = packages.my-project;
    });
}
