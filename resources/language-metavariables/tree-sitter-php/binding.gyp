{
  "targets": [
    {
      "target_name": "tree_sitter_php_binding",
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "php/src"
      ],
      "sources": [
        "php/src/parser.c",
        "php/src/scanner.c",
        "php_only/src/parser.c",
        "php_only/src/scanner.c",
        "bindings/node/binding.cc"
      ],
      "cflags_c": [
        "-std=c99"
      ]
    }
  ]
}
