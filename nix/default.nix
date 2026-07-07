{
  stdenv ? pkgs.stdenv,
  lib ? pkgs.lib,
  pkgs ? import <nixpkgs> { },
  crossPkgs ? pkgs,
  ...
}: let
  toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./../rust-toolchain.toml);
  cargoManifest = fromTOML (builtins.readFile ./../Cargo.toml);
  rustPkgs = if crossPkgs == null then pkgs else crossPkgs;

  buildInputs = with pkgs; [
    libGL
    fontconfig
    pkgs.stdenv.cc.cc.lib
    rustPlatform.bindgenHook
    libxcb
    libx11
    libxcb
    freetype
    libxkbcommon

    wayland

    libjpeg
    vulkan-loader
  ];

  libPath = with pkgs; lib.makeLibraryPath [
    libGL
    libglvnd
    fontconfig
    libxkbcommon
    libx11
    libxcursor
    libxext
    libxrandr
    libxi
    wayland
    vulkan-loader
  ];

  simplemojiPkg = (rustPkgs.rustPlatform.buildRustPackage.override { stdenv = rustPkgs.clangStdenv; }) (finalAttrs: {
    pname = cargoManifest.package.name;
    version = cargoManifest.package.version;
    src = ./..;
    cargoLock.lockFile = ./../Cargo.lock;
    doCheck = false;
    nativeBuildInputs = with pkgs;
      [
        pkg-config
        python3
        makeWrapper
        removeReferencesTo

        rustPlatform.bindgenHook
        autoPatchelfHook
      ] ++ lib.optionals stdenv.buildPlatform.isDarwin [
        libiconv
        cctools.libtool
      ];
    runtimeDependencies = with pkgs;
      [ noto-fonts-color-emoji ]
      ++ lib.optionals stdenv.isLinux [
        wayland
        libxkbcommon
      ];

    makeWrapperArgs = [
      "--prefix LD_LIBRARY_PATH : ${libPath}"
    ];
    inherit buildInputs;

    postFixup = ''
      remove-references-to -t "$SKIA_SOURCE_DIR" $out/bin/simplemoji
      patchelf --set-rpath "${libPath}" $out/bin/${cargoManifest.package.name}
    '';
    disallowedReferences = [ finalAttrs.SKIA_SOURCE_DIR ];

    SKIA_NINJA_COMMAND = "${pkgs.ninja}/bin/ninja";
    SKIA_GN_COMMAND = "${pkgs.gn}/bin/gn";
    SKIA_ENABLE_TOOLS = "false";
    SKIA_LIBRARY_DIR = "${pkgs.skia}/lib";
    SKIA_SOURCE_DIR =
      let
        repo = pkgs.fetchFromGitHub {
          owner = "rust-skia";
          repo = "skia";
          # see rust-skia:skia-bindings/Cargo.toml#package.metadata skia
          tag = "m150-0.98.1";
          hash = "sha256-h/TFrGlqDur7bvIc9CBqDBwSJOQBk0N52/jwle3ay7c=";
        };
        # The externals for skia are taken from skia/DEPS
        externals = pkgs.linkFarm "skia-externals" (
          lib.mapAttrsToList (name: value: {
            inherit name;
            path = pkgs.fetchgit value;
          }) (lib.importJSON ./skia-externals.json)
        );
      in
      pkgs.runCommand "source" { } ''
        cp -R ${repo} $out
        chmod -R +w $out
        ln -s ${externals} $out/third_party/externals
      '';
  });
in {
  # `nix run`
  apps.default = {
    type = "app";
    program = "${simplemojiPkg}/bin/simplemoji";
  };
  # `nix build`
  packages.default = simplemojiPkg;
  # `nix develop`
  devShells.default = pkgs.mkShell {
    packages = with pkgs; [
        toolchain
        pkg-config
        cargo-dist
        cargo-release
        git-cliff
      ] ++ buildInputs;
    LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
  };
}
