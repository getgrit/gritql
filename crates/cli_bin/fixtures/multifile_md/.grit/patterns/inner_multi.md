---
title: Inner Multi
---

This is where the actual multifile is defined, used by staged_multi.md.

tags: #good

```grit
engine marzano(0.1)
language js

multifile {
    bubble($x) file($name, $body) where $body <: contains `foo($x)`,
    bubble($x) file($name, $body) where $body <: contains `bar($x)` => `baz($x)`
}
```
