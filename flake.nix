{
  inputs = {
    utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages."${system}";
      naersk-lib = naersk.lib."${system}";
    in rec {
      # `nix build`
      packages.yasps = naersk-lib.buildPackage {
        pname = "yasps";
        root = ./.;
      };
      defaultPackage = packages.yasps;

      # `nix run`
      apps.yasps = utils.lib.mkApp {
        drv = packages.yasps;
      };
      defaultApp = apps.yasps;

      # `nix develop`
      devShell = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [ rustc cargo ];
      };
    });
}
