with import <nixpkgs> {};
mkShell {
  buildInputs = [
    cargo-udeps
    gdb # required for rust-gdb
    gh
    rustup
    rust-analyzer
    yamllint
  ];
}
