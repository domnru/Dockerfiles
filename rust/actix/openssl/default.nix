{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    git
    rustup
    rust-analyzer
    openssl
    pkg-config
  ];
}
