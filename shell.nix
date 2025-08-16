with import <nixpkgs> {};
mkShell {
  buildInputs = [
    gdb # required for rust-gdb
    rustup
    rust-analyzer
    yamllint
  ];
}
