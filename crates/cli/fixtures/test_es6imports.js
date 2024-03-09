require('./side-effects-only-1.js');
const { named1, alias2: named2, alias3: named3 } = require('path/to/file/1');
const alias4 = require('npm-lib-1').named4;
const entireModule = require('npm-lib-2').default;
const fs = require('fs');
