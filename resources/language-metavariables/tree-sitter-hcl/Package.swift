// swift-tools-version:5.3

import PackageDescription

let package = Package(
    name: "TreeSitterHCL",
    platforms: [.macOS(.v10_13), .iOS(.v11)],
    products: [
        .library(name: "TreeSitterHCL", targets: ["TreeSitterHCL"]),
    ],
    dependencies: [],
    targets: [
        .target(name: "TreeSitterHCL",
                path: ".",
                exclude: [
                    "binding.gyp",
                    "bindings",
                    "Cargo.toml",
                    "CHANGELOG.md",
                    "docs",
                    "example",
                    "grammar.js",
                    "LICENSE",
                    "package.json",
                    "README.md",
                    "shell.nix",
                    "src/grammar.json",
                    "src/node-types.json",
                    "test",
                ],
                sources: [
                    "src/parser.c",
                    "src/scanner.c",
                ],
                publicHeadersPath: "bindings/swift",
                cSettings: [.headerSearchPath("src")],
                linkerSettings: [.linkedLibrary("c++")])
    ]
)
