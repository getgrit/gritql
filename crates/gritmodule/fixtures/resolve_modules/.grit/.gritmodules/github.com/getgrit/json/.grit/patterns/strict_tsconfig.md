---
title: Strict tsconfig
---

# Strict tsconfig

Adds `"strict": true, "allowJs": true, "checkJs": false` from a tsconfig's `compilerOptions`, and then removes existing redundant options (such as `noImplicitAny`).

tags: #js, #ts, #tsconfig, #strict

```grit
engine marzano(0.1)
language json

pair(key=`"compilerOptions"`, $value) where {
    $value <: object($properties),
    $newOptions = [],

    $properties <: maybe contains pair(key=`"noImplicitAny"`, value=`false` => `true`),
    $properties <: maybe contains pair(key=`"strict"`, value=`false` => `true`),
    if ($properties <: contains pair(key=`"strict"`, value=$strict)) {
        $strict <: maybe `false` => `true`
    } else {
        $newOptions += `"strict": true`
    },

    // These are all included by default; ideally we'd delete, but can't, so just mark as true
    $properties <: maybe contains pair(key=`"noImplicitThis"`) => .,
    $properties <: maybe contains pair(key=`"alwaysStrict"`) => .,
    $properties <: maybe contains pair(key=`"strictBindCallApply"`) => .,
    $properties <: maybe contains pair(key=`"strictNullChecks"`) => .,
    $properties <: maybe contains pair(key=`"strictFunctionTypes"`) => .,
    $properties <: maybe contains pair(key=`"strictPropertyInitialization"`) => .,

    // allow JS
    if ($properties <: contains pair(key=`"allowJs"`, value=$allow_js)) {
        $allow_js <: maybe `false` => `true`
    } else {
        $newOptions += `"allowJs": true`
    },

    // check JS
    if ($properties <: contains pair(key=`"checkJs"`, value=$check_js)) {
        $check_js <: maybe `true` => `false`
    } else {
        $newOptions += `"checkJs": false`
    },

    if (!$newOptions <: []) {
        $joined = join(list=$newOptions, separator=", "),
        $properties => `$joined, $properties`
    }
}
```

## Transform standard tsconfig.json

```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "outDir": "../../dist/out-tsc",
    "types": ["node"],
    "foo": "bar"
  },
  "exclude": ["**/*.spec.ts"],
  "include": ["**/*.ts"]
}
```

```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "allowJs": true,
    "checkJs": false,
    "outDir": "../../dist/out-tsc",
    "types": ["node"],
    "foo": "bar"
  },
  "exclude": ["**/*.spec.ts"],
  "include": ["**/*.ts"]
}
```

## Handles redundant options

```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "outDir": "../../dist/out-tsc",
    "types": ["node"],
    "foo": "bar",
    "baz": "raz"
  },
  "exclude": ["**/*.spec.ts"],
  "include": ["**/*.ts"]
}
```

```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "allowJs": true,
    "checkJs": false,
    "outDir": "../../dist/out-tsc",
    "types": ["node"],
    "foo": "bar",
    "baz": "raz"
  },
  "exclude": ["**/*.spec.ts"],
  "include": ["**/*.ts"]
}
```

## Handles existing strict

```json
{
  "compilerOptions": {
    "target": "es5",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "noImplicitAny": false,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "baseUrl": ".",
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
```

```json
{
  "compilerOptions": {
    "checkJs": false,
    "target": "es5",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "forceConsistentCasingInFileNames": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "node",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "preserve",
    "incremental": true,
    "noImplicitAny": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "baseUrl": ".",
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx", ".next/types/**/*.ts"],
  "exclude": ["node_modules"]
}
```

## Bugfix $decl <: false

```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "outDir": "../../dist/out-tsc",
    "module": "commonjs",
    "types": ["node", "express"],
    "allowJs": true
  },
  "exclude": ["jest.config.ts", "**/*.spec.ts", "**/*.test.ts"],
  "include": ["**/*.ts"]
}
```

```json
{
  "extends": "./tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "checkJs": false,
    "outDir": "../../dist/out-tsc",
    "module": "commonjs",
    "types": ["node", "express"],
    "allowJs": true
  },
  "exclude": ["jest.config.ts", "**/*.spec.ts", "**/*.test.ts"],
  "include": ["**/*.ts"]
}
```
