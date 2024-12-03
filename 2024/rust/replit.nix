{ pkgs }: {
  deps = with pkgs; [
    rustc
    rustfmt
    cargo
    cargo-edit
    rust-analyzer
  ];
}
