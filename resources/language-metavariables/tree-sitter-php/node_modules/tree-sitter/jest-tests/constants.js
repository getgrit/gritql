module.exports = {
INPUT: `
  const Parser = require(".");
  const Javascript = require("tree-sitter-javascript");
  const jsParser = new Parser();
`,

// from running runit.js
OUTPUT: "(program (lexical_declaration (variable_declarator name: (identifier) value: (call_expression function: (identifier) arguments: (arguments (string (string_fragment)))))) (lexical_declaration (variable_declarator name: (identifier) value: (call_expression function: (identifier) arguments: (arguments (string (string_fragment)))))) (lexical_declaration (variable_declarator name: (identifier) value: (new_expression constructor: (identifier) arguments: (arguments)))))"
}
