{
  description = "Standar cross compile flake for Rust Lang Projects";
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    fenix.url = "github:nix-community/fenix/monthly";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    crane.url = "github:ipetkov/crane";
  };
  outputs = inputs @ {
    flake-parts,
    fenix,
    nixpkgs,
    flake-utils,
    crane,
    self,
    ...
  }:
    inputs.flake-parts.lib.mkFlake
    {
      inherit inputs;
    }
    {
      systems = ["x86_64-linux"];
      perSystem = {
        config,
        pkgs,
        system,
        ...
      }: let
        # inherit (pkgs) lib;
        # Toolchain
        toolchain = with fenix.packages.${system};
          combine [
            stable.cargo
            stable.clippy
            stable.rust-src
            stable.rustc
            stable.rustfmt
            targets.x86_64-unknown-linux-gnu.stable.rust-std
          ];
        craneLib = crane.lib.${system}.overrideToolchain toolchain;

        src = craneLib.cleanCargoSource (craneLib.path ./.);
        commonArgs = {
          inherit src;
          buildInputs = with pkgs; [
              openssl.dev
              pkg-config

              libinput
              libxkbcommon
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              xorg.libX11
          ];
        };
        # Compile all artifacts for x86_64-unknown-linux-gnu
        linuxArtifacts = craneLib.buildDepsOnly (commonArgs
          // {
            CARGO_BUILD_TARGET = "x86_64-unknown-linux-gnu";
            doCheck = false;
          });

        # Compile app for x86_64-unknown-linux-gnu
        linuxApp = craneLib.buildPackage (
          commonArgs
          // {
            doCheck = false;
            cargoArtifacts = linuxArtifacts;
          }
        );
      in {
        # nix build
        packages = {
          default = linuxApp;
        };

        # nix run
        apps = {
          default = flake-utils.lib.mkApp {
            drv = linuxApp;
          };
        };

        # nix develop
        devShells.default = craneLib.devShell {
          packages = with pkgs; [
            toolchain
            fontconfig
            noto-fonts-color-emoji
          ];
        };
      };
    };
}