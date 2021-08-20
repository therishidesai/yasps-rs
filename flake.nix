{
  description = "A shared memory pub/sub system";

  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.abacus-c =
      let pkgs = import nixpkgs {
            system = "x86_64-linux";
          };
          stdenv = pkgs.clangStdenv;
      in stdenv.mkDerivation {
        pname = "yasps";
        version = "0.0.1";
        src = ./.;

        buildPhase = ''
                   # make
                   # make samples
        '';

        installPhase = ''
                     mkdir -p $out/bin
                     # cp yasps-example $out/bin/.
        '';
      };

    # Specify the default package
    defaultPackage.x86_64-linux = self.packages.x86_64-linux.yasps;
  };
}
