````markdown {% fileName=".grit/patterns/remove_console_log.md" %}
---
tags: [optional, tags, here]
---
# Remove console.log

Remove console.log in production code.

```grit
`console.log($_)` => .
```

## Test case one

This is the first test case. You can include an explanation of the case here.

```typescript
console.error("keep this");
console.log('remove this!');
```

It is fine to include additional descriptive text around the test cases.
This is often used to explain the context of the test case, or to explain a convention.

```typescript
console.error("keep this");

```
````
