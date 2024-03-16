---
title: Multifile
---

tags: #good

```grit
engine marzano(0.1)
language js

pattern clam() {
    multifile {
        bubble($x) file($name, $body) where $body <: contains `foo($x)`,
        bubble($x) file($name, $body) where $body <: contains `bar($x)` => `baz($x)`
    }
}

pattern mussel() {
    clam()
}

mussel()
```