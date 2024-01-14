{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "shell-rust-wasm";
  nativeBuildInputs = with pkgs; [
    # requires the rust-bin overlay
    (rust-bin.stable.latest.default.override {
      targets = [ "wasm32-unknown-unknown" ];
    })
    wasm-pack
    wasm-bindgen-cli
    binaryen
  ];
}
