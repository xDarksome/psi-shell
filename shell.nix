{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell rec {
    buildInputs = with pkgs; [
      pkg-config

      cargo
      clippy
      rustc

      gtk3
      gtk-layer-shell

      librsvg
    ];
  }