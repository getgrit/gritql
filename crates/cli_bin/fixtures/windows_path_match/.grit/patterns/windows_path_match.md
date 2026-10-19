---
name: windows_path_match
language: js
engine: marzano(0.1)
---

file($name, $body) where {
  $name <: `src/index.js`
}
