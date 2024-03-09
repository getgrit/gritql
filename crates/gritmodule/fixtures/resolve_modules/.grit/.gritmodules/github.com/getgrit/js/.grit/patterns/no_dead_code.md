---
title: Remove unreachable code
---

Remove unreachable code found after `return` / `throw` / `continue` or `break` statements.

tags: #good, #SE

```grit
engine marzano(0.1)
language js

statement_block($statements) where {
    $deleting = "false",
    $statements <: some bubble($deleting) $s where {
        if ($deleting <: "true") {
            $s => .
        } else {
            // we start deleting
            if ($s <: or { throw_statement() , continue_statement() , return_statement() }) {
                $deleting = "true",
            }
        }
    }
}
```

## Remove code after return

```javascript
function f() {
  return 3;
  1 + 1;
}
```

```typescript
function f() {
  return 3;
}
```

## Remove code after return, multiline

```javascript
function f() {
  foo();
  return 3;
  1 + 1;
}
```

```typescript
function f() {
  foo();
  return 3;
}
```

## Don't exit a scope

```javascript
function f() {
  if (a) {
    return 3;
  }
  1 + 1;
}
```
