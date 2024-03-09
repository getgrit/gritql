// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "TreeSitterHTML",
    products: [
        .library(name: "TreeSitterHTML", targets: ["TreeSitterHTML"]),
    ],
    dependencies: [],
    targets: [
        .target(name: "TreeSitterHTML",
                path: ".",
                exclude: [
                    "binding.gyp",
                    "bindings",
                    "Cargo.toml",
                    "grammar.js",
                    "LICENSE",
                    "package.json",
                    "README.md",
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
