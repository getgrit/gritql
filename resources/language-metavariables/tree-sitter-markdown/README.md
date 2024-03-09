# tree-sitter-markdown
A markdown parser for [tree-sitter].

![](https://github.com/MDeiml/tree-sitter-markdown/blob/split_parser/contrib/screenshot.png)

The parser is designed to read markdown according to the [CommonMark Spec],
but some extensions to the spec from different sources such as [Github flavored
markdown] are also included. These can be toggled on or off at compile time.
For specifics see [Extensions](#extensions)

## Goals

Even though this parser has existed for some while and obvious issues are
mostly solved, there are still lots of inaccuarcies in the output. These stem
from restricting a complex format such as markdown to the quite restricting
tree-sitter parsing rules.

As such it is not recommended to use this parser where correctness is
important. The main goal for this parser is to provide syntactical information
for syntax highlighting in parsers such as [neovim] and [helix].

## Contributing

All contributions are welcome. For details refer to [CONTRIBUTING.md].

## Extensions

Extensions can be enabled at compile time through environment variables. Some
of them are on by default, these can be disabled with the environment variable
`NO_DEFAULT_EXTENSIONS`.

| Name | Environment variable | Specification | Default | Also enables |
|:----:|:--------------------:|:-------------:|:-------:|:------------:|
| Github flavored markdown | `EXTENSION_GFM` | [link](https://github.github.com/gfm/) | ✓ | Task lists, strikethrough, pipe tables |
| Task lists | `EXTENSION_TASK_LIST` | [link](https://github.github.com/gfm/#task-list-items-extension-) | ✓ |  |
| Strikethrough | `EXTENSION_STRIKETHROUGH` | [link](https://github.github.com/gfm/#strikethrough-extension-) | ✓ |  |
| Pipe tables | `EXTENSION_PIPE_TABLE` | [link](https://github.github.com/gfm/#tables-extension-) | ✓ |  |
| YAML metadata | `EXTENSION_MINUS_METADATA` | [link](https://gohugo.io/content-management/front-matter/) | ✓ |  |
| TOML metadata | `EXTENSION_PLUS_METADATA` | [link](https://gohugo.io/content-management/front-matter/) | ✓ |  |
| Tags | `EXTENSION_TAGS` | [link](https://help.obsidian.md/Editing+and+formatting/Tags#Tag+format) |  |  |
| Wiki Link | `EXTENSION_WIKI_LINK` | [link](https://help.obsidian.md/Linking+notes+and+files/Internal+links) |  |  |

## Usage in Editors

For guides on how to use this parser in a specific editor, refer to that
editor's specific documentation, e.g.
* [neovim](https://github.com/nvim-treesitter/nvim-treesitter)
* [helix](https://docs.helix-editor.com/guides/adding_languages.html)

## Standalone usage

To use the two grammars, first parse the document with the block
grammar. Then perform a second parse with the inline grammar using
`ts_parser_set_included_ranges` to specify which parts are inline content.
These parts are marked as `inline` nodes. Children of those inline nodes should
be excluded from these ranges. For an example implementation see `lib.rs` in
the `bindings` folder.

### Usage with WASM

Unfortunately using this parser with WASM/web-tree-sitter does not work out of the box at the moment. This is because the parser uses some C functions that are not exported by tree-sitter by default. To fix this you can statically link the parser to tree-sitter. See also https://github.com/tree-sitter/tree-sitter/issues/949, https://github.com/MDeiml/tree-sitter-markdown/issues/126, and https://github.com/MDeiml/tree-sitter-markdown/issues/93

[CommonMark Spec]: https://spec.commonmark.org/
[Github flavored markdown]: https://github.github.com/gfm/
[tree-sitter]: https://tree-sitter.github.io/tree-sitter/
[neovim]: https://neovim.io/
[helix]: https://helix-editor.com/
[CONTRIBUTING.md]: https://github.com/MDeiml/tree-sitter-markdown/blob/split_parser/CONTRIBUTING.md
