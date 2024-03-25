{
  "targets": [
    {
      "target_name": "tree_sitter_runtime_binding",
      "dependencies": ["tree_sitter"],
      "sources": [
        "src/binding.cc",
        "src/conversions.cc",
        "src/language.cc",
        "src/logger.cc",
        "src/node.cc",
        "src/parser.cc",
        "src/query.cc",
        "src/tree.cc",
        "src/tree_cursor.cc",
        "src/util.cc",
      ],
      "include_dirs": [
        "vendor/tree-sitter/lib/include",
        "<!(node -e \"require('nan')\")",
      ],
      'cflags': [
        '-std=c++17'
      ],
      'cflags_cc': [
        '-std=c++17'
      ],
      'conditions': [
        ['OS=="mac"', {
          'xcode_settings': {
            'MACOSX_DEPLOYMENT_TARGET': '10.9',
            'CLANG_CXX_LANGUAGE_STANDARD': 'c++17',
            'CLANG_CXX_LIBRARY': 'libc++',
          },
        }],
        ['OS=="win"', {
          'msvs_settings': {
            'VCCLCompilerTool': {
              'AdditionalOptions': [
                '/std:c++17',
              ],
              'RuntimeLibrary': 0,
            },
          },
        }],
        ['OS == "linux"', {
          'cflags_cc': [
            '-Wno-cast-function-type'
          ]
        }],
        ['runtime=="electron"', {
          'defines': ['NODE_RUNTIME_ELECTRON=1']
        }],
      ],
    },
    {
      "target_name": "tree_sitter",
      'type': 'static_library',
      "sources": [
        "vendor/tree-sitter/lib/src/lib.c"
      ],
      "include_dirs": [
        "vendor/tree-sitter/lib/src",
        "vendor/tree-sitter/lib/include",
      ],
      "cflags": [
        "-std=c99"
      ]
    }
  ],
  'variables': {
    'runtime%': 'node',
    'openssl_fips': '',
    'v8_enable_pointer_compression%': 0,
    'v8_enable_31bit_smis_on_64bit_arch%': 0,
  },
  'conditions': [
      ['runtime=="electron"', { 'defines': ['NODE_RUNTIME_ELECTRON=1'] }],
  ]
}
