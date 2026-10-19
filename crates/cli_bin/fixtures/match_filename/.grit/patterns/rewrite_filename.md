---
name: rewrite_filename
language: js
engine: marzano(0.1)
---

```grit
file($name, $body) where {
  $name <: `index.js`
}
```
