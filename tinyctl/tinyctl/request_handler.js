// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

'use strict';

const request = require('request');

const deploy = (address, functionObject) => {
  return new Promise((resolve, reject) => {
    request.post({url: address, json: functionObject}, (err, response, body) => {
      if (err) reject(err);
      resolve(body);
    })
  })
}

module.exports = deploy;