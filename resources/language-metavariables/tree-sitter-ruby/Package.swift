// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "TreeSitterRuby",
    platforms: [.macOS(.v10_13), .iOS(.v11)],
    products: [
        .library(name: "TreeSitterRuby", targets: ["TreeSitterRuby"]),
    ],
    dependencies: [],
    targets: [
        .target(name: "TreeSitterRuby",
                path: ".",
                exclude: [
                    "test",
                    "script",
                    "bindings",
                    "binding.gyp",
                    "src/node-types.json",
                    "src/grammar.json",
                    "grammar.js",
                    "LICENSE",
                    "README.md",
                    "Cargo.toml",
                    "Makefile",
                    "package.json",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.cc",
                ],
                resources: [
                    .copy("queries")
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")])
    ]
)
