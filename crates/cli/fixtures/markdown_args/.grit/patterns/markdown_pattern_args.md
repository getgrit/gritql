---
title: Argument please
---

Raise a useful error when doing this.

```grit
engine marzano(0.1)
language js

pattern markdown_pattern_args($output) {
  `console.log($_)` => $output
}

markdown_pattern_args(output="test_case")


```

## One test

```javascript
console.log("hello world");
```

```typescript
test_case
```
