# Force flushing

Prevent returning without flushing the emitter in `apply_pattern.rs`.

```grit
language rust

pattern flushable_return() {
  `$expression?` where {
    $expression <: not includes "flush",
    $expression <: not within `let mut emitter: MessengerVariant = $_;`,
    $expression => `flushable_unwrap!(emitter, $expression)`
  }
}

file(name=includes "apply_pattern.rs", $body) where {
  $body <: contains `pub(crate) async fn run_apply_pattern(
      $_
  ) -> Result<()> { $func }` where {
    $func <: contains flushable_return()
  }
}
```
