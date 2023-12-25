{ lib
, rustPlatform
, fetchFromGitHub
, pkg-config
, libxkbcommon
, stdenv
, darwin
, wayland
, xorg
}:

rustPlatform.buildRustPackage rec {
  pname = "simplemoji";
  version = "0.1.6";

  src = fetchFromGitHub {
    owner = "SergioRibera";
    repo = "simplemoji";
    rev = "v${version}";
    hash = "sha256-Ty+q7orXIID/yuSpXH7U/YEkhAUjLkzYP5k1+V5QDLY=";
  };

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  postPatch = ''
    ln -s ${./Cargo.lock} Cargo.lock
  '';

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    libxkbcommon
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.AppKit
    darwin.apple_sdk.frameworks.CoreFoundation
    darwin.apple_sdk.frameworks.CoreGraphics
    darwin.apple_sdk.frameworks.Foundation
  ] ++ lib.optionals stdenv.isLinux [
    wayland
    xorg.libX11
    xorg.libxcb
  ];

  meta = with lib; {
    description = "Fast Application for look your amazing emojis write in Rust";
    homepage = "https://github.com/SergioRibera/simplemoji";
    license = with licenses; [ asl20 mit ];
    maintainers = with maintainers; [ ];
    mainProgram = "simplemoji";
  };
}
