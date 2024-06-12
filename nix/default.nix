let
  inherit
    (builtins)
    currentSystem
    fromJSON
    readFile
    ;
  getFlake = name:
    with (fromJSON (readFile ../flake.lock)).nodes.${name}.locked; {
      inherit rev;
      outPath = fetchTarball {
        url = "https://github.com/${owner}/${repo}/archive/${rev}.tar.gz";
        sha256 = narHash;
      };
    };
in
  {
    system ? currentSystem,
    pkgs ? import (getFlake "nixpkgs") {localSystem = {inherit system;};},
    lib ? pkgs.lib,
    crane,
    cranix,
    fenix,
    stdenv ? pkgs.stdenv,
    ...
  }: let
    # fenix: rustup replacement for reproducible builds
    # toolchain = fenix.${system}.fromToolchainFile { dir = ./..; };
    toolchain = fenix.${system}.fromToolchainFile {
      file = ./../rust-toolchain.toml;
      sha256 = "sha256-opUgs6ckUQCyDxcB9Wy51pqhd0MPGHUVbwRKKPGiwZU=";
    };
    # crane: cargo and artifacts manager
    craneLib = crane.${system}.overrideToolchain toolchain;
    # cranix: extends crane building system with workspace bin building and Mold + Cranelift integrations
    cranixLib = craneLib.overrideScope' (cranix.${system}.craneOverride);

    # buildInputs for Simplemoji
    buildInputs = with pkgs; [
      fontconfig.dev
      libxkbcommon.dev
      wayland
      libxkbcommon
      xorg.libxcb
      xorg.libX11
      xorg.libXcursor
      xorg.libXrandr
      xorg.libXi
    ];

    # Base args, need for build all crate artifacts and caching this for late builds
    commonArgs = {
      src = ./..;
      doCheck = false;
      nativeBuildInputs =
        [pkgs.pkg-config]
        ++ lib.optionals stdenv.buildPlatform.isDarwin [
          pkgs.libiconv
        ];
      runtimeDependencies = with pkgs;
        lib.optionals stdenv.isLinux [
          wayland
          libxkbcommon
          noto-fonts-color-emoji
        ];
      inherit buildInputs;
    };

    # simplemoji artifacts
    simplemojiDeps = cranixLib.buildCranixDepsOnly commonArgs;

    # Lambda for build packages with cached artifacts
    packageArgs = targetName:
      commonArgs
      // {
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER = "${stdenv.cc.targetPrefix}cc";
        CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_RUNNER = "qemu-aarch64";
        HOST_CC = "${stdenv.cc.nativePrefix}cc";
        cargoArtifacts = simplemojiDeps;
        workspaceTargetName = targetName;
      };

    # Build packages and `nix run` apps
    simplemojiPkg = cranixLib.buildCranixBundle (packageArgs "simplemoji");
  in {
    # `nix run`
    apps = rec {
      simplemoji = simplemojiPkg.app;
      default = simplemoji;
    };
    # `nix build`
    packages = rec {
      simplemoji = simplemojiPkg.pkg;
      default = simplemoji;
    };
    # `nix develop`
    devShells.default = cranixLib.devShell {
      packages = with pkgs;
        [
          toolchain
          pkg-config
          cargo-dist
          cargo-release

          libxkbcommon
          noto-fonts-color-emoji
        ]
        ++ buildInputs;
      LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
      PKG_CONFIG_PATH = "${pkgs.fontconfig.dev}/lib/pkgconfig";
    };
  }
