---
title: Aspect ratio
---

```grit
language css

`a { $props }` where {
  $props <: contains `aspect-ratio: $x`
}
```

## Matches the right selector and declaration block

```css
a {
  width: calc(100% - 80px);
  aspect-ratio: 1/2;
  font-size: calc(10px + (56 - 10) * ((100vw - 320px) / (1920 - 320)));
}

#some-id {
  some-property: 5px;
}

a.b ~ c.d {
}
.e.f + .g.h {
}

@font-face {
  font-family: 'Open Sans';
  src: url('/a') format('woff2'), url('/b/c') format('woff');
}
```

```css
a {
  width: calc(100% - 80px);
  aspect-ratio: 1/2;
  font-size: calc(10px + (56 - 10) * ((100vw - 320px) / (1920 - 320)));
}

#some-id {
  some-property: 5px;
}

a.b ~ c.d {
}
.e.f + .g.h {
}

@font-face {
  font-family: 'Open Sans';
  src: url('/a') format('woff2'), url('/b/c') format('woff');
}
```
