# Markdown Pattern

This is to show how to use markdown patterns.

```grit
pattern show_this_one() {
  `foo` => `bar`
}

private pattern hide_this() {
  `foods` => `calories`
}

or {
  `foo` => `bar`,
  show_this_one(),
  hide_this()
}
```