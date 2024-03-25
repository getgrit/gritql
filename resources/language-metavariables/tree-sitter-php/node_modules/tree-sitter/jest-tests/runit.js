const Parser = require("..");
const constants = require("./constants");
const Javascript = require("tree-sitter-javascript");
const jsParser = new Parser();
jsParser.setLanguage(Javascript);

const code = jsParser.parse(constants.INPUT)
const output = code.rootNode.toString()
console.log(output);
