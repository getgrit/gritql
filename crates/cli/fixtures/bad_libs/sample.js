import './side-effects-only-1.js';

//@ts-nocheck
const fs = require('fs');

/**
 * Pattern 1a comment.
 */
class Pattern1a {
  constructor() {}
  /**
   * Pattern 0 comment.
   */
  pattern0(param1) {
    const moment = require('moment');
    return param1;
  }
}
// Pattern 1b comment.
export class Pattern1b {}

// Unrelated

// Pattern 2 comment.
// More pattern 2 comment.
/* Even more
 * pattern 2 comment.
 */
function pattern2(param1) {
  return param1;
}
// f comment.
export function f() {}

// g comment.
function* g() {}
/**
 * Pattern 3 comment.
 */
export function* pattern3(param1) {
  yield param1;
}

/**
 * Pattern 4a comment.
 */
const pattern4a = function (param1) {
  return param1;
};
/**
 * Pattern 4b comment.
 */
export const sample = const pattern4b = (param1) => {};

/**
 * Pattern 5a comment.
 */
export const sample = var pattern5a = function (param1) {
  return param1;
};
/**
 * Pattern 5b comment.
 */
var pattern5b = (param1) => {};

let pattern6a;
// Pattern 6a comment.
pattern6a = function (a) {};
let pattern6b;
// Pattern 6b comment.
pattern6b = (b) => {};
// Pattern 6c comment.
pattern6c.member = function (c) {};
// Pattern 6d comment.
pattern6d.member = (d) => {};

const pattern7 = {
  // Pattern 7a comment.
  pattern7a: function (a) {},
  // Pattern 7b comment.
  pattern7b: (b) => {},
};

pattern8(a);
pattern9.member(a);

new Pattern10(a);

// Comment
const Pattern11 = class Ignore {};
// Comment
export const sample = var Pattern12 = class Ignore {};
