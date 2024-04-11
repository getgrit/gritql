// @ts-check
"use-strict";

const TreeSitter = require("web-tree-sitter");

async function initialize_tree_sitter() {
  await TreeSitter.init();
  globalThis.Parser = TreeSitter;
  globalThis.Language = TreeSitter.Language;
}

module.exports = {
  initialize_tree_sitter,
};
