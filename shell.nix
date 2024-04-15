{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup pkgs.pkg-config pkgs.openssl
  ] ++ (if pkgs.stdenv.targetPlatform.isDarwin then [
    pkgs.libiconv
    pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
  ] else []);

  OPENSSL_DEV=pkgs.openssl.dev;
}
