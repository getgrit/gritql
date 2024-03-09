// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "TreeSitterMarkdown",
    platforms: [.macOS(.v10_13), .iOS(.v11)],
    products: [
        .library(name: "TreeSitterMarkdown", targets: ["TreeSitterMarkdown", "TreeSitterMarkdownInline"]),
    ],
    dependencies: [],
    targets: [
        .target(name: "TreeSitterMarkdown",
                path: "tree-sitter-markdown",
                exclude: [
                    "corpus",
                    "grammar.js",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.c",
                ],
                resources: [
                    .copy("queries")
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")]),
        .target(name: "TreeSitterMarkdownInline",
                path: "tree-sitter-markdown-inline",
                exclude: [
                    "corpus",
                    "grammar.js",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.c",
                ],
                resources: [
                    .copy("queries")
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")])
    ]
)
