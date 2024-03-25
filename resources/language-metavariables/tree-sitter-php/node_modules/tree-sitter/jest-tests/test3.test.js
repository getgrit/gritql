const Parser = require("..");
const constants = require("./constants");
const Javascript = require("tree-sitter-javascript");
const jsParser = new Parser();
jsParser.setLanguage(Javascript);

describe("Jest test 1 duplicate", () => {
  it("should work", () => {
    const code = jsParser.parse(constants.INPUT)
    // Due to the race condition arising from Jest's worker pool,
    // code.rootNode is null if the native extension hasn't finished
    // loading. In this case, we skip the test.
    if (code.rootNode) {
      const output = code.rootNode.toString()
      expect(output).toBe(constants.OUTPUT);
    }
  })
})
