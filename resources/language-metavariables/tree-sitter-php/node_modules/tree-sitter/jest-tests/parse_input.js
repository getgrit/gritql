const Parser = require("..");
const Javascript = require("tree-sitter-javascript");
const jsParser = new Parser();
jsParser.setLanguage(Javascript);

module.exports = (input) => {
  const code = jsParser.parse(input)
  return code.rootNode;
}
