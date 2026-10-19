{ pkgs ? import <nixpkgs> { }
, lib ? pkgs.lib
, src ? lib.cleanSource ./.
}:

pkgs.tree-sitter-grammars.tree-sitter-nix.overrideAttrs (old: {
  name = "tree-sitter-nix-dev";
  version = "dev";
  inherit src;

  doCheck = true;
  checkInputs = [
    pkgs.tree-sitter
    pkgs.nodejs
  ];
  checkPhase = ''
    HOME=$(mktemp -d) tree-sitter test
  '';
})
