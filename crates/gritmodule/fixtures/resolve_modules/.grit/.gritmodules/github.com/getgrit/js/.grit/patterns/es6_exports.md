---
title: Prefer ES6-style exports over module.exports
---

Converts CommonJS `module.exports` to ES6-style exports.

tags: #js, #es6, #migration, #cjs, #commonjs

```grit
engine marzano(0.1)
language js

or {
    and {
        // handle a default export of an object by exporting the individual original definitions
        `module.exports = { $vals }` where {
            $new_export = "",
            // it's only safe to remove the overall export if every property is individually exported
            $vals <: some bubble($new_export) $prop where {
                $prop <: or {
                    shorthand_property_identifier() as $name where { $value = $name },
                    pair(key=$name, $value)
                },
                or {
                    $program <: contains or {
                        // does not handle difficult trying to match a sublist of the module.exports
                        `const $name = require($val).$foo` => `export { $foo as $name } from $val`,
                        `const $name = require($val)` => `export { default as $name } from $val`,
                        `const $name = require($val).default` => `export { default as $name } from $val`,
                        or {
                            `let $name = $val`,
                            `var $name = $val`,
                            `const $name = $val`,
                            function_declaration($name)
                        } as $match => `export $match`
                    },
                    if ($value <: $name) {
                        $new_export += `export { $name };\n`
                    } else {
                        $new_export += `export const $name = $value;\n`
                    }
                }
            }
        } => `$new_export`
    },
    // handle other default exports
    `module.exports = $export` => `export default $export`,
    // Handle individually named exports
    `module.exports.$name = $export` => `export const $name = $export;\n`
}
```

## Transform direct exports

```js
module.exports.king = '9';
```

```js
export const king = '9';
```

## Transform default exports

```js
async function createTeam() {
  console.log('cool');
}

const addTeamToOrgSubscription = () => console.log('cool');

module.exports = {
  createTeam,
  addTeamToOrgSubscription,
};
```

```js
export async function createTeam() {
  console.log('cool');
}

export const addTeamToOrgSubscription = () => console.log('cool');
```

### Keep inline values in tact

```js
const king = '9';

module.exports = {
  king,
  queen: '8',
};
```

```js
export const king = '9';

export const queen = '8';
```

### Work on

```js
const c1 = require('./mod1');
const c2 = require('./mod2');
const c3 = require('./mod3');
const myDefaultConst = require('./mod4').default;
const myRenamed = require('mod5').originalName;
const { sub1, sub2 } = require('mod5'); // not handled

module.exports = { c1, c2, c3, myDefaultConst, myRenamed, sub1, sub2 };
```

```js
export { default as c1 } from './mod1';
export { default as c2 } from './mod2';
export { default as c3 } from './mod3';
export { default as myDefaultConst } from './mod4';
export { originalName as myRenamed } from 'mod5';
const { sub1, sub2 } = require('mod5'); // not handled

export { sub1 };
export { sub2 };
```
