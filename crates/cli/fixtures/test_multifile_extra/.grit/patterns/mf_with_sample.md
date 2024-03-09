---
title: multi_file_with_sample
---

This is a test for the multi-file feature. It should not pass.

```grit
engine marzano(0.1)
language js

multifile {
    bubble($x) file($name, $body) where $body <: contains `foo($x)`,
    bubble($x) file($name, $body) where $body <: contains `bar($x)` => `baz($x)`
}
```

## Sample

Samples can't be tested for multi-file patterns.

```js
// @filename: apple.js
foo(1)

// @filename: mango.js
bar(1)
bar(3)
```

```js
// @filename: apple.js
foo(1)

// @filename: mango.js
baz(1)
bar(3)

// @filename: watermelon.js
foo(1)
```

