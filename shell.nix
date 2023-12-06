{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    tree
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
  ];
  
  RUST_BACKTRACE = 1;
}
