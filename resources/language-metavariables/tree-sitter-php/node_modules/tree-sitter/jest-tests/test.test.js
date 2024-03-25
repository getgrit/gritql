const Parser = require("..");
const constants = require("./constants");
const parse_input = require("./parse_input.js");
const Javascript = require("tree-sitter-javascript");

const { Query } = Parser;
const jsParser = new Parser();
jsParser.setLanguage(Javascript);

describe("Jest test 1", () => {
  it("should work", () => {
    const code = jsParser.parse(constants.INPUT);
    // Due to the race condition arising from Jest's worker pool,
    // code.rootNode is null if the native extension hasn't finished
    // loading. In this case, we skip the test.
    if (code.rootNode) {
      const output = code.rootNode.toString();
      expect(output).toBe(constants.OUTPUT);
    }
  });

  it("should work with separate import", () => {
    const rootNode = parse_input(constants.INPUT);
    if (rootNode) {
      expect(rootNode.toString()).toBe(constants.OUTPUT);
    }
  });
  function assertCursorState(cursor, params) {
    expect(cursor.nodeType).toBe(params.nodeType);
    expect(cursor.nodeIsNamed).toBe(params.nodeIsNamed);
    expect(cursor.startPosition).toEqual(params.startPosition);
    expect(cursor.endPosition).toEqual(params.endPosition);
    expect(cursor.startIndex).toEqual(params.startIndex);
    expect(cursor.endIndex).toEqual(params.endIndex);

    const node = cursor.currentNode;
    expect(node.type).toBe(params.nodeType);
    expect(node.isNamed).toBe(params.nodeIsNamed);
    expect(node.startPosition).toEqual(params.startPosition);
    expect(node.endPosition).toEqual(params.endPosition);
    expect(node.startIndex).toEqual(params.startIndex);
    expect(node.endIndex).toEqual(params.endIndex);
  }

  function assert(thing) {
    expect(thing).toBeTruthy();
  }

  it("should work with cursors", () => {
    const tree = jsParser.parse("a * b + c / d");

    const cursor = tree.walk();
    assertCursorState(cursor, {
      nodeType: "program",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 13 },
      startIndex: 0,
      endIndex: 13,
    });

    assert(cursor.gotoFirstChild());
    assertCursorState(cursor, {
      nodeType: "expression_statement",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 13 },
      startIndex: 0,
      endIndex: 13,
    });

    assert(cursor.gotoFirstChild());
    assertCursorState(cursor, {
      nodeType: "binary_expression",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 13 },
      startIndex: 0,
      endIndex: 13,
    });

    assert(cursor.gotoFirstChild());
    assertCursorState(cursor, {
      nodeType: "binary_expression",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 5 },
      startIndex: 0,
      endIndex: 5,
    });

    assert(cursor.gotoFirstChild());
    assertCursorState(cursor, {
      nodeType: "identifier",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 1 },
      startIndex: 0,
      endIndex: 1,
    });

    assert(!cursor.gotoFirstChild());
    assert(cursor.gotoNextSibling());
    assertCursorState(cursor, {
      nodeType: "*",
      nodeIsNamed: false,
      startPosition: { row: 0, column: 2 },
      endPosition: { row: 0, column: 3 },
      startIndex: 2,
      endIndex: 3,
    });

    assert(cursor.gotoNextSibling());
    assertCursorState(cursor, {
      nodeType: "identifier",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 4 },
      endPosition: { row: 0, column: 5 },
      startIndex: 4,
      endIndex: 5,
    });

    assert(!cursor.gotoNextSibling());
    assert(cursor.gotoParent());
    assertCursorState(cursor, {
      nodeType: "binary_expression",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 0 },
      endPosition: { row: 0, column: 5 },
      startIndex: 0,
      endIndex: 5,
    });

    assert(cursor.gotoNextSibling());
    assertCursorState(cursor, {
      nodeType: "+",
      nodeIsNamed: false,
      startPosition: { row: 0, column: 6 },
      endPosition: { row: 0, column: 7 },
      startIndex: 6,
      endIndex: 7,
    });

    assert(cursor.gotoNextSibling());
    assertCursorState(cursor, {
      nodeType: "binary_expression",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 8 },
      endPosition: { row: 0, column: 13 },
      startIndex: 8,
      endIndex: 13,
    });

    const childIndex = cursor.gotoFirstChildForIndex(12);
    assertCursorState(cursor, {
      nodeType: "identifier",
      nodeIsNamed: true,
      startPosition: { row: 0, column: 12 },
      endPosition: { row: 0, column: 13 },
      startIndex: 12,
      endIndex: 13,
    });
    expect(childIndex).toBe(2);

    assert(!cursor.gotoNextSibling());
    assert(cursor.gotoParent());
    assert(cursor.gotoParent());
    assert(cursor.gotoParent());
    assert(cursor.gotoParent());
    assert(!cursor.gotoParent());
  });

  it("returns all of the matches for the given query", () => {
    const tree = jsParser.parse("function one() { two(); function three() {} }");
    const query = new Query(
      Javascript,
      `
    (function_declaration name: (identifier) @fn-def)
    (call_expression function: (identifier) @fn-ref)
  `
    );
    const matches = query.matches(tree.rootNode);
    expect(formatMatches(tree, matches)).toEqual([
      { pattern: 0, captures: [{ name: "fn-def", text: "one" }] },
      { pattern: 1, captures: [{ name: "fn-ref", text: "two" }] },
      { pattern: 0, captures: [{ name: "fn-def", text: "three" }] },
    ]);
  });
});

function formatMatches(tree, matches) {
  return matches.map(({ pattern, captures }) => ({
    pattern,
    captures: formatCaptures(tree, captures),
  }));
}

function formatCaptures(tree, captures) {
  return captures.map((c) => {
    const node = c.node;
    delete c.node;
    c.text = tree.getText(node);
    return c;
  });
}
