{ pkgs ? import <nixpkgs> {}}:
with pkgs; mkShell {
  buildInputs = [
    rustc
    cargo
    rust-analyzer
    gdb
  ];
  
  shellHook = ''
    echo Monuey Counter Project
  '';
}
