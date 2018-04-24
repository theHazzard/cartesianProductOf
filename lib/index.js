const cartesianProductOfNative = require('../native').cartesianProductOf;
const cartesianProductOf = require('./productOf');

let first = 100;
let second = 150;
let third = 200;

first = Array.from(Array(first)).map(() => ({
  a: 'sarasa',
  b: 'masSarasa',
  d: {
    a: 55,
  },
}));

second = Array.from(Array(second)).map(() => ({
  a: 'sarasa',
  b: 'masSarasa',
  d: {
    a: 55,
  },
}));

third = Array.from(Array(third)).map(() => ({
  a: 'sarasa',
  b: 'masSarasa',
  d: {
    a: 55,
  },
}));

console.time('normal');
const results = cartesianProductOf(...[first, second, third]);
console.timeEnd('normal');

console.time('nativo');
const resultsNative = cartesianProductOfNative(...[first, second, third]);
console.timeEnd('nativo');
console.log(resultsNative);
debugger;
