---
title: Rewrite `x == -0` ⇒ `Object.is(x, -0)`
---

Convert any equality check with `-0` to the more precise `Object.is`.

Details on [on StackOverflow](https://stackoverflow.com/questions/7223359/are-0-and-0-the-same).

tags: #SD

```grit
engine marzano(1.0)
language js

binary_expression($left, $operator, $right) as $exp where {
    $operator <: or {
        or { "==", "===" } where $exp => `Object.is($left, -0)`,
        or { "!=", "!==" } where $exp => `!Object.is($left, -0)`
    }
}
```

## Basic example

```javascript
if (x == -0 || x !== -0) {
  foo(;
}
```

```typescript
if (Object.is(x, -0) || !Object.is(x, -0)) {
  foo();
}
```

