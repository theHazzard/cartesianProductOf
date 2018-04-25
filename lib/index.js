const cartesianProductOfNative = require('../native').cartesianProductOf;
const cartesianProductOf = require('./productOf');
const expect = require('chai').expect;

describe('Recieve 0 params', () => {
  let result = [[]];
  it('Should return an element of empty element', () => {
    expect(result).to.deep.equal(cartesianProductOfNative())
  });
});

describe('Recieve 1 params', () => {
  let first = Array.from(Array(5)).map(() => ({ a: 'First Object' }));
  let result = cartesianProductOf(first);
  it('Should return an element of empty elements', () => {
    expect(result).to.deep.equal(cartesianProductOfNative(first));
  });
});


describe('Recieve 2 params', () => {
  let first = Array.from(Array(5)).map(() => ({ a: 'First Object' }));
  let second = Array.from(Array(7)).map(() => ({ b: 'Second Object' }));
  let result = cartesianProductOf(...[first, second]);
  it('Should return an element of empty elements', () => {
    expect(result).to.deep.equal(cartesianProductOfNative(...[first, second]));
  });
});

describe('Recieve n params', () => {
  let first = Array.from(Array(5)).map(() => ({ a: 'First Object' }));
  let second = Array.from(Array(7)).map(() => ({ b: 'Second Object' }));
  let third = Array.from(Array(7)).map(() => ({ b: 'Second Object' }));
  let fourth = Array.from(Array(7)).map(() => ({ b: 'Second Object' }));
  let result = cartesianProductOf(...[first, second, third, fourth]);
  it('Should return an element of empty elements', () => {
    expect(result).to.deep.equal(cartesianProductOfNative(...[first, second, third, fourth]));
  });
});
