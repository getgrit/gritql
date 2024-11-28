# tree-sitter-c-sharp

[![CI][ci]](https://github.com/tree-sitter/tree-sitter-c-sharp/actions/workflows/ci.yml)
[![discord][discord]](https://discord.gg/w7nTvsVJhm)
[![matrix][matrix]](https://matrix.to/#/#tree-sitter-chat:matrix.org)
[![crates][crates]](https://crates.io/crates/tree-sitter-c-sharp)
[![npm][npm]](https://www.npmjs.com/package/tree-sitter-c-sharp)
[![pypi][pypi]](https://pypi.org/project/tree-sitter-c-sharp)

C# grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter) based upon the Roslyn grammar with changes in order to:

- Deal with differences between the parsing technologies
- Work around some bugs in that grammar
- Handle `#if`, `#else`, `#elif`, `#endif` blocks
- Support syntax highlighting/parsing of fragments
- Simplify the output tree
- Reduce parser state count and complexity
- Be in-line with tree-sitter's convention where applicable

### Status

Comprehensive supports C# 1 through 13.0 with the following exception:

- [ ] `async`, `var` and `await` cannot be used as identifiers everywhere they are valid

### References

- [Official C# 6 Language Spec](https://github.com/dotnet/csharplang/blob/master/spec/) provides chapters that formally define the language grammar.
- [Roslyn C# language grammar export](https://github.com/dotnet/roslyn/blob/master/src/Compilers/CSharp/Portable/Generated/CSharp.Generated.g4)
- [SharpLab](https://sharplab.io) (web-based syntax tree playground based on Roslyn)

[ci]: https://img.shields.io/github/actions/workflow/status/tree-sitter/tree-sitter-c-sharp/ci.yml?logo=github&label=CI
[discord]: https://img.shields.io/discord/1063097320771698699?logo=discord&label=discord
[matrix]: https://img.shields.io/matrix/tree-sitter-chat%3Amatrix.org?logo=matrix&label=matrix
[npm]: https://img.shields.io/npm/v/tree-sitter-c-sharp?logo=npm
[crates]: https://img.shields.io/crates/v/tree-sitter-c-sharp?logo=rust
[pypi]: https://img.shields.io/pypi/v/tree-sitter-c-sharp?logo=pypi&logoColor=ffd242
