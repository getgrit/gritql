---
title: Compare `null` using  `===` or `!==`
---

Comparing to `null` needs a type-checking operator (=== or !==), to avoid incorrect results when the value is `undefined`.

tags: #good

```grit
engine marzano(0.1)
language js

// We use the syntax-tree node binary_expression to capture all expressions where $a and $b are operated on by "==" or "!=".
// This code takes advantage of Grit's allowing us to nest rewrites inside match conditions and to match syntax-tree fields on patterns.
binary_expression($operator, $left, $right) where {
    $operator <: or  { "==" => `===` , "!=" => `!==` },
    or { $left <: `null`, $right <: `null`}
}

```

```

```

## `$val == null` => `$val === null`

```javascript
if (val == null) {
  done();
}
```

```typescript
if (val === null) {
  done();
}
```

## `$val != null` => `$val !== null`

```javascript
if (val != null) {
  done();
}
```

```typescript
if (val !== null) {
  done();
}
```

## `$val != null` => `$val !== null` into `while`

```javascript
while (val != null) {
  did();
}
```

```typescript
while (val !== null) {
  did();
}
```

## Do not change `$val === null`

```javascript
if (val === null) {
  done();
}
```

## Do not change `$val !== null`

```
while (val !== null) {
  doSomething();
}
```
