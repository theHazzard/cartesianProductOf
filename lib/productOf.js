module.exports = function cartesianProductOf(...args) {
  return Array.prototype.reduce.call(args, (a, b) => {
    const ret = [];
    a.forEach((aItem) => {
      b.forEach((bItem) => {
        ret.push(aItem.concat([bItem]));
      });
    });
    return ret;
  }, [[]]);
}
