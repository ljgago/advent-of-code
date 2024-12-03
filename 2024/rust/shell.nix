{ pkgs ? import <nixpkgs> { } }:

let
  mkShell = pkgs.mkShell;
in
mkShell {
  buildInputs = with pkgs; [
    rustc
    rustfmt
    cargo
    cargo-edit
    rust-analyzer

    # Cute prompt
    starship
  ];

  shellHook = ''
    eval "$(starship init bash)"
  '';
}
