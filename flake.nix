{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-utils,
    rust-overlay,
    ...
  }@inputs:
  # Iterate over Arm, x86 for MacOs 🍎 and Linux 🐧
    flake-utils.lib.eachSystem (flake-utils.lib.defaultSystems) (
      system: let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        simplemojiBundle = crossPkgs: import ./nix {
          inherit system pkgs crossPkgs;
        };

        strategies = pkgs.callPackage ./strategies.nix {};

        mkCrossPkgs = { arch, os, ... }:
          if os == "windows"
          then pkgs.pkgsCross.mingwW64
          else let
            cross = arch + "-" + os;
            crossSystem = pkgs.lib.systems.elaborate cross;
          in
            import nixpkgs {
              inherit overlays;
              crossSystem =
                if cross != "x86_64-linux"
                then crossSystem
                else null;
              localSystem = system;
            };

        allTargets =
          builtins.concatMap (
            pkg:
              builtins.map (arch: {
                inherit (pkg) pname;
                inherit (arch) arch os target formats;
                features = pkg.features or [];
              })
              strategies.archs
          )
          strategies.packages;

        generatedPackages = builtins.listToAttrs (pkgs.lib.flatten (map (
            { target, arch, os, formats, pname, ... }@args:
              map (format: let
                crossPkgs = mkCrossPkgs args;
                fmt = pkgs.lib.strings.replaceStrings ["."] ["_"] format;
                drv = (simplemojiBundle crossPkgs).packages.default;
                pkgKey = "${os}-${arch}-${fmt}";
              in {
                name = pkgKey;
                value = (pkgs.callPackage ./bundle.nix {
                  inherit drv format pname;
                  target = { inherit os arch pname; };
                }) // args;
              })
              formats
          )
          allTargets));

        generatedMatrixJson = builtins.toJSON (pkgs.lib.flatten (map (
            { pname, arch, os, formats, ... }:
              map (format: let
                fmt = pkgs.lib.strings.replaceStrings ["."] ["_"] format;
                name = "${os}-${arch}-${fmt}";
              in {
                inherit pname arch os format;
                bundle = "${name}";
              })
              formats
          )
          allTargets));
      in {
        devShells = (simplemojiBundle null).devShells;
        apps = (simplemojiBundle null).apps // {
          matrix = {
            type = "app";
            program = toString (pkgs.writeScript "generate-matrix" ''
              #!/bin/sh
              echo '${generatedMatrixJson}'
            '');
          };

          list-variants = {
            type = "app";
            program = toString (pkgs.writeScript "generate-all-variants" ''
              #!/bin/sh
              echo '${generatedMatrixJson}'
            '');
          };
        };
        packages = generatedPackages;
    }) // (flake-utils.lib.eachDefaultSystemPassThrough (system: {
        # Overlays
        overlays.default = final: prev: {
          simplemoji = inputs.self.packages.${prev.system}.default;
        };
        # nixosModules
        nixosModules = {
          default = import ./nix/nixos-module.nix;
          home-manager = import ./nix/hm-module.nix;
        };
      }
    ));
}
