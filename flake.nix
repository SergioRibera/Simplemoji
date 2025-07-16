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
        simplemojiBundle = import ./nix {
          inherit system pkgs;
        };
      in {
        inherit (simplemojiBundle) apps devShells;
        packages = simplemojiBundle.packages // {
          x86_64-linux-tar = pkgs.callPackage ./bundle.nix {
            target = { os = "linux"; arch = "x86_64"; pname = "simplemoji"; };
            format = "tar.gz";
            pname = "simplemoji";
            drv = simplemojiBundle.packages.default;
          };
        };
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
