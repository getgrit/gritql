let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  name = "env";
  buildInputs = with pkgs; [
    gdb
    valgrind
    nodejs
    tree-sitter
    emscripten
  ];
}

