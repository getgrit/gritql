---
title: Compare `null` using  `===` or `!==`
tags: ["good", "exclude2"]
---

Comparing to `null` needs a type-checking operator (=== or !==), to avoid incorrect results when the value is `undefined`.

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

## `$val == null` => `$val === null` to change

```javascript
if (val == null) {
  done();
  cnonsole.log("This should be added to the output by patterns test --update");
}
```

```typescript
if (val === null) {
  done();
}
```

## `$val != null` => `$val !== null` to channge

```javascript
if (val != null) {
  done();
  console.log("This should be added as well");
}
```

```typescript
if (val !== null) {
  done();
}
```

## `$val != null` => `$val !== null` into `while` not change

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

## Do not change `$val === null` not change

```javascript
if (val === null) {
  done();
}
```

## Do not change `$val !== null` not change

```
while (val !== null) {
  doSomething();
}
```
