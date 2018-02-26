// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

'use strict';

module.exports = (param, context) => {
  let returnObj;
  if (param && param.name) {
    returnObj = { message: `Hello ${param.name}!` };
  } else {
    returnObj = { message: 'Hello world!' };
  }
  return returnObj;
}
