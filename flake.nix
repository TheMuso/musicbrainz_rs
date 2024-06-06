{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-24.05";
  };

  outputs = { self, nixpkgs, ... }:
    let
      forAllSystems = nixpkgs.lib.genAttrs [
        "aarch64-linux"
        "x86_64-linux"
      ];
    in
      let
        runtimeDeps = pkgs: with pkgs; [ openssl ];
        buildDeps = pkgs: with pkgs; [ cargo pkg-config rustPlatform.bindgenHook ];
        devDeps = pkgs: with pkgs; [ clippy gdb rustfmt ];

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        msrv = cargoToml.package.rust-version;

        rustPackage = { pkgs, features ? "" }:
          (pkgs.makeRustPlatform {
            cargo = pkgs.cargo;
            rustc = pkgs.rustc;
          }).buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
            buildFeatures = features;
            buildInputs = runtimeDeps pkgs;
            nativeBuildInputs = buildDeps pkgs;
            # Uncomment if your cargo tests require networking or otherwise
            # don't play nicely with the Nix build sandbox:
            # doCheck = false;
          };

        mkDevShell = { pkgs, rustc }:
          pkgs.mkShell {
            shellHook = ''
              export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
            '';
            buildInputs = runtimeDeps pkgs;
            nativeBuildInputs = buildDeps pkgs ++ devDeps pkgs ++ [ rustc ];
          };
      in {
        packages = forAllSystems (system:
          let pkgs = nixpkgs.legacyPackages.${system};
          in rec {
            musicbrainz-rs = rustPackage { inherit pkgs; };
            default = musicbrainz-rs;
          }
        );

        devShells = forAllSystems (system:
          let pkgs = nixpkgs.legacyPackages.${system};
          in rec {
            default = mkDevShell { inherit pkgs; rustc = pkgs.rustc; };
          }
        );
      };
}
