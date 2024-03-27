# tree-sitter-c

This crate provides a PHP grammar for the [tree-sitter][] parsing library. To
use this crate, add it to the `[dependencies]` section of your `Cargo.toml`
file. (Note that you will probably also need to depend on the
[`tree-sitter`][tree-sitter crate] crate to use the parsed result in any useful
way.)

```toml
[dependencies]
tree-sitter = "0.20.10"
tree-sitter-php = "0.22.2"
```

Typically, you will use the [language][language func] function to add this
grammar to a tree-sitter [Parser][], and then use the parser to parse some code:

```rust
use tree_sitter::Parser;

fn main() {
    let code = r#"
    <?php

    function double(int $x) {
        return $x * 2;
    }
"#;
    let mut parser = Parser::new();
    parser.set_language(tree_sitter_php::language()).expect("Error loading PHP grammar");
    let parsed = parser.parse(code, None);
    println!("{:#?}", parsed);
}
```

If you have any questions, please reach out to us in the [tree-sitter
discussions] page.

[language func]: https://docs.rs/tree-sitter-php/*/tree_sitter_php/fn.language.html
[Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
[tree-sitter]: https://tree-sitter.github.io/
[tree-sitter crate]: https://crates.io/crates/tree-sitter
[tree-sitter discussions]: https://github.com/tree-sitter/tree-sitter/discussions
