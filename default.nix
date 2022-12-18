{ lib, pkgs, rustPlatform, pkg-config, openssl, darwin, ... }:
rustPlatform.buildRustPackage rec {
  pname = "bingwallpaper";
  version = "0.1.0";
  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ] ++ (lib.optionals pkgs.stdenv.isDarwin
    [ darwin.apple_sdk.frameworks.Security ]);
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkg-config";
  src = ./.;
  sourceRoot = "bingwallpaper";
  cargoSha256 = "sha256-urpTGmtYYR6HXGt/VFrHiYmlkXyBvJ5ba/X+t6Uv9VM=";
}
