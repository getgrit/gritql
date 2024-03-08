# tree-sitter-vue

[![npm](https://img.shields.io/npm/v/tree-sitter-vue.svg)](https://www.npmjs.com/package/tree-sitter-vue)
[![build](https://img.shields.io/travis/com/ikatyang/tree-sitter-vue/master.svg)](https://travis-ci.com/ikatyang/tree-sitter-vue/builds)

Vue ([Vue v2.6.0 Template Syntax](https://vuejs.org/v2/guide/syntax.html)) grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter)

_Note: This grammar is not responsible for parsing embedded languages, see [Multi-language Documents](http://tree-sitter.github.io/tree-sitter/using-parsers#multi-language-documents) for more info._

[Changelog](https://github.com/ikatyang/tree-sitter-vue/blob/master/CHANGELOG.md)

## Install

```sh
npm install tree-sitter-vue tree-sitter
```

## Usage

```js
const Parser = require("tree-sitter");
const Vue = require("tree-sitter-vue");

const parser = new Parser();
parser.setLanguage(Vue);

const sourceCode = `
<template>
  Hello, <a :[key]="url">{{ name }}</a>!
</template>
`;

const tree = parser.parse(sourceCode);
console.log(tree.rootNode.toString());
// (component
//   (template_element
//     (start_tag
//       (tag_name))
//       (text)
//       (element
//         (start_tag
//           (tag_name)
//           (directive_attribute
//             (directive_name)
//             (directive_dynamic_argument
//               (directive_dynamic_argument_value))
//             (quoted_attribute_value
//               (attribute_value))))
//         (interpolation
//           (raw_text))
//         (end_tag
//           (tag_name)))
//       (text)
//     (end_tag
//       (tag_name))))
```

## License

MIT © [Ika](https://github.com/ikatyang)
