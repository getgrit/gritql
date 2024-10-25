# CLI build

```
cargo build
cargo install --path .
marzano --help
```

## Usage

1. check to run a pattern on a file

   ```
   marzano check --pattern=test_jsx.grit test_jsx.js
   ```

## Tests

You can run the tests with:

```
cargo test
```

## Building

Note that the built binary is relative to the _workspace_ root. It will be at `../../../target/release/grit`.

```
yarn build
```
