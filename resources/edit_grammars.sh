#!/bin/bash

set -e

# assumes that this script is run from marzano/resources directory
rm -rf language-metavariables
mkdir language-metavariables
rsync -r -l language-submodules/. language-metavariables --exclude={.git*,tree-sitter-*/example,tree-sitter-*/test,tree-sitter-*/corpus}
cd language-metavariables

cd tree-sitter-toml && npm install regexp-util && npx tree-sitter generate && cd ..

# we need to make sure all the languages use the same version of tree-sitter
for cargo in */[Cc]argo.toml ; do
    sed -i '' -e 's/tree-sitter = ".*"/tree-sitter = "~0.20"/g' "$cargo"
done

cp ../metavariable-grammars/css-metavariable-grammar.js tree-sitter-css/grammar.js
cp ../metavariable-grammars/json-metavariable-grammar.js tree-sitter-json/grammar.js
cp ../metavariable-grammars/solidity-metavariable-grammar.js tree-sitter-solidity/grammar.js
cp ../metavariable-grammars/sql-metavariable-grammar.js tree-sitter-sql/grammar.js
cp ../metavariable-grammars/hcl-metavariable-grammar.js tree-sitter-hcl/make_grammar.js
cp ../metavariable-grammars/python-metavariable-grammar.js tree-sitter-python/grammar.js
cp ../metavariable-grammars/markdown-common-metavariable-grammar.js tree-sitter-markdown/common/grammar.js
cp ../metavariable-grammars/markdown-block-metavariable-grammar.js tree-sitter-markdown/tree-sitter-markdown/grammar.js
cp ../metavariable-grammars/markdown-inline-metavariable-grammar.js tree-sitter-markdown/tree-sitter-markdown-inline/grammar.js
cp ../metavariable-grammars/javascript-metavariable-grammar.js tree-sitter-javascript/grammar.js
cp ../metavariable-grammars/java-metavariable-grammar.js tree-sitter-java/grammar.js
cp ../metavariable-grammars/rust-metavariable-grammar.js tree-sitter-rust/grammar.js
cp ../metavariable-grammars/go-metavariable-grammar.js tree-sitter-go/grammar.js
cp ../metavariable-grammars/vue-metavariable-grammar.js tree-sitter-vue/grammar.js
cp ../metavariable-grammars/yaml-metavariable-grammar.js tree-sitter-yaml/grammar.js
cp ../metavariable-grammars/yaml-metavariable-scanner.cc tree-sitter-yaml/src/scanner.cc
cp ../metavariable-grammars/toml-metavariable-grammar.js tree-sitter-toml/grammar.js

# typescript is special
# we edit the package.json to point to our local version of the js grammar
cp ../metavariable-grammars/typescript-package.json tree-sitter-typescript/package.json
# typescript defines a typescript and tsx grammar so the grammar we care about is in common/define-grammar.js
cp ../metavariable-grammars/typescript-metavariable-define-grammar.js tree-sitter-typescript/common/define-grammar.js

# vue package.json needs to be updated to use a newer version of nan, and local version of html grammar
cp ../metavariable-grammars/vue-package.json tree-sitter-vue/package.json

# tree-sitter hangs on c-sharp and typescript has a special file structure for tsx and typescript.
# wanted to * and exclude c-sharp and type-script but couldn't get exclusion glob to work, so just listed all the languages.

# TODO MARKDOWN INLINE AND MARKDOWN SHOULD PROBABLY BE BUILT TOGETHER
for dir in {tree-sitter-css,tree-sitter-go,tree-sitter-hcl,tree-sitter-html,tree-sitter-java,tree-sitter-javascript,tree-sitter-json,tree-sitter-markdown/tree-sitter-markdown,tree-sitter-markdown/tree-sitter-markdown-inline,tree-sitter-python,tree-sitter-ruby,tree-sitter-rust,tree-sitter-solidity,tree-sitter-yaml,tree-sitter-toml};
    do (cd $dir && npx tree-sitter generate && npx tree-sitter build-wasm && echo "Generated grammar for ${PWD##*/}" ) &
done
cd "tree-sitter-sql" && npx tree-sitter generate && echo "Generated grammar for ${PWD##*/}" &
wait
cd tree-sitter-typescript && yarn && yarn build && echo "Generated grammar for ${PWD##*/}"
cd tsx && npx tree-sitter build-wasm;
cd ..
cd typescript && npx tree-sitter build-wasm;
cd ../..
cd tree-sitter-vue && yarn && yarn prepack && npx tree-sitter build-wasm && echo "Generated grammar for ${PWD##*/}"
cd ..

cp ../metavariable-grammars/cc_build.rs tree-sitter-yaml/bindings/rust/build.rs
cp ../metavariable-grammars/cc_build.rs tree-sitter-vue/bindings/rust/build.rs
cp ../metavariable-grammars/c_build.rs tree-sitter-sql/bindings/rust/build.rs

# I suck at bash scripting and couldn't figure out how to do this in a loop
# need the language name, but also need bash to expand the file literal
#marzano resources

cp ../../../../vendor/tree-sitter-gritql/src/node-types.json ../node-types/gritql-node-types.json
cp tree-sitter-c-sharp/src/node-types.json ../node-types/csharp-node-types.json
cp tree-sitter-css/src/node-types.json ../node-types/css-node-types.json
cp tree-sitter-go/src/node-types.json ../node-types/go-node-types.json
cp tree-sitter-hcl/src/node-types.json ../node-types/hcl-node-types.json
cp tree-sitter-html/src/node-types.json ../node-types/html-node-types.json
cp tree-sitter-java/src/node-types.json ../node-types/java-node-types.json
cp tree-sitter-json/src/node-types.json ../node-types/json-node-types.json
cp tree-sitter-markdown/tree-sitter-markdown/src/node-types.json ../node-types/markdown-block-node-types.json
cp tree-sitter-markdown/tree-sitter-markdown-inline/src/node-types.json ../node-types/markdown-inline-node-types.json
cp tree-sitter-python/src/node-types.json ../node-types/python-node-types.json
cp tree-sitter-ruby/src/node-types.json ../node-types/ruby-node-types.json
cp tree-sitter-rust/src/node-types.json ../node-types/rust-node-types.json
cp tree-sitter-solidity/src/node-types.json ../node-types/solidity-node-types.json
cp tree-sitter-yaml/src/node-types.json ../node-types/yaml-node-types.json
cp tree-sitter-javascript/src/node-types.json ../node-types/javascript-node-types.json
cp tree-sitter-typescript/typescript/src/node-types.json ../node-types/typescript-node-types.json
cp tree-sitter-typescript/tsx/src/node-types.json ../node-types/tsx-node-types.json
cp tree-sitter-sql/src/node-types.json ../node-types/sql-node-types.json
cp tree-sitter-vue/src/node-types.json ../node-types/vue-node-types.json
cp tree-sitter-toml/src/node-types.json ../node-types/toml-node-types.json

# move the wasm parsers to the wasm-parser directory
mv tree-sitter-css/tree-sitter-css.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-css.wasm
mv tree-sitter-go/tree-sitter-go.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-go.wasm
mv tree-sitter-hcl/tree-sitter-hcl.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-hcl.wasm
mv tree-sitter-html/tree-sitter-html.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-html.wasm
mv tree-sitter-java/tree-sitter-java.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-java.wasm
mv tree-sitter-json/tree-sitter-json.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-json.wasm
# mv tree-sitter-markdown/tree-sitter-markdown/tree-sitter-markdown.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-markdown-block.wasm
mv tree-sitter-markdown/tree-sitter-markdown-inline/tree-sitter-markdown_inline.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-markdown_inline.wasm
mv tree-sitter-python/tree-sitter-python.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-python.wasm
mv tree-sitter-ruby/tree-sitter-ruby.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-ruby.wasm
mv tree-sitter-rust/tree-sitter-rust.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-rust.wasm
mv tree-sitter-solidity/tree-sitter-solidity.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-solidity.wasm
mv tree-sitter-yaml/tree-sitter-yaml.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-yaml.wasm
mv tree-sitter-javascript/tree-sitter-javascript.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-javascript.wasm
mv tree-sitter-typescript/typescript/tree-sitter-typescript.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-typescript.wasm
mv tree-sitter-typescript/tsx/tree-sitter-tsx.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-tsx.wasm
mv tree-sitter-vue/tree-sitter-vue.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-vue.wasm
mv tree-sitter-toml/tree-sitter-toml.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-toml.wasm
# we skip wasm generation as it's too expensive
# mv tree-sitter-sql/tree-sitter-sql.wasm ../../wasm-bindings/wasm_parsers/tree-sitter-sql.wasm

# Modify existing C/C++ compiler flag to ignore all warnings
find . -name "build.rs" -exec sed -i '' -e 's/Wno-unused-parameter/w/g' {} \;