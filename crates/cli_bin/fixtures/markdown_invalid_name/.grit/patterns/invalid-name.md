---
title: Rewrite `x == -0` ⇒ `Object.is(x, -0)`
---

Convert any equality check with `-0` to the more precise `Object.is`.

Details on [on StackOverflow](https://stackoverflow.com/questions/7223359/are-0-and-0-the-same).

tags: #SD

```grit
engine marzano(1.0)
language js

`$a = $b` => `console.log($b)`
```
