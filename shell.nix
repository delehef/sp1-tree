{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
  ] ++ (if pkgs.stdenv.targetPlatform.isDarwin then [
    pkgs.libiconv
    pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
  ] else []);

  OPENSSL_DEV=pkgs.openssl.dev;
}
