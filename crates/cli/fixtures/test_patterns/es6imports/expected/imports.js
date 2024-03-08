import * as defaultImport from '../../shared/default'
import { something, another, } from './lib';
import assert as assert from 'chai'
import config as conf from 'chai'
import starImport from 'star'
import './side-effects-only-1.js';
import { named1, alias2 as named2, alias3 as named3, } from 'path/to/file/1';
import named4 as alias4 from 'npm-lib-1'
import * as entireModule from 'npm-lib-2'
import fs from 'fs'
