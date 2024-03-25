// swift-tools-version:5.3
import PackageDescription

let package = Package(
  name: "TreeSitterPHP",
  products: [
    .library(name: "TreeSitterPHP", targets: ["TreeSitterPHP"]),
  ],
  dependencies: [],
  targets: [
    .target(
      name: "TreeSitterPHP",
      path: ".",
      exclude: [
      ],
      sources: [
        "php/src/parser.c",
        "php/src/scanner.c",
        "php_only/src/parser.c",
        "php_only/src/scanner.c",
      ],
      resources: [
        .copy("queries")
      ],
      publicHeadersPath: "bindings/swift",
      cSettings: [.headerSearchPath("php/src")]
    )
  ]
)
