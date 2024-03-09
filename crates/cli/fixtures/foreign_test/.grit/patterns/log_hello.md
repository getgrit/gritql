---
title: Change `console.log` to say hello
---

```grit
engine marzano(0.1)
language js

`console.log($arg)` where {
  $hi = sayHi(),
  $arg => $hi,
}
```

## Transforms a simple `console.log` statement

```javascript
// Do not remove this
console.error('foo');
console.log('foo');
```

```javascript
// Do not remove this
console.error('foo');
console.log("Hello!");
```
