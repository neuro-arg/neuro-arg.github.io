{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "shell-hugo";
  nativeBuildInputs = with pkgs; [ hugo ];
}
