{
  stdenv ? pkgs.stdenv,
  lib ? pkgs.lib,
  pkgs,
  ...
}: let
  toolchain = (pkgs.rust-bin.fromRustupToolchainFile ./../rust-toolchain.toml);

  # buildInputs for Simplemoji
  buildInputs = with pkgs; [
    pkg-config
    fontconfig.dev
    libxkbcommon.dev
    wayland
    xorg.libxcb
    xorg.libX11
    xorg.libXcursor
    xorg.libXrandr
    xorg.libXi
    libgcc
  ];

  simplemojiPkg = pkgs.rustPlatform.buildRustPackage {
    pname = "simplemoji";
    version = "0.1.0";
    src = ./..;
    cargoLock.lockFile = ./../Cargo.lock;
    doCheck = false;
    nativeBuildInputs =
      [ toolchain ]
      ++ lib.optionals stdenv.isLinux [
        pkgs.pkg-config
        pkgs.autoPatchelfHook
      ]
      ++ lib.optionals stdenv.buildPlatform.isDarwin [
        pkgs.libiconv
      ];
    runtimeDependencies = with pkgs;
      [ noto-fonts-color-emoji ]
      ++ lib.optionals stdenv.isLinux [
        wayland
        libxkbcommon
      ];

    postFixup = lib.optionalString stdenv.isLinux ''
      patchelf --set-rpath "${lib.makeLibraryPath buildInputs}" $out/bin/simplemoji
    '';

    dontWrapQtApps = true;
    makeWrapperArgs = [
      "--prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath buildInputs}"
      "--prefix PATH : ${lib.makeBinPath [ pkgs.wayland ]}"
    ];
    inherit buildInputs;
  };
in {
  # `nix run`
  apps.default = simplemojiPkg;
  # `nix build`
  packages.default = simplemojiPkg;
  # `nix develop`
  devShells.default = pkgs.mkShell {
    packages = with pkgs; [
        toolchain
        pkg-config
        cargo-dist
        cargo-release

        # libxkbcommon
        # noto-fonts-color-emoji
      ] ++ buildInputs;
    LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";
    PKG_CONFIG_PATH = "${pkgs.fontconfig.dev}/lib/pkgconfig";
  };
}
