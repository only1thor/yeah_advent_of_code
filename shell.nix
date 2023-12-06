{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
  ];
  shellHook = ''
    exec fish 
  '';
  RUST_BACKTRACE = 1;
}