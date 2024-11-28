import XCTest
import SwiftTreeSitter
import TreeSitterCSharp

final class TreeSitterCSharpTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_c_sharp())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading C# grammar")
    }
}
