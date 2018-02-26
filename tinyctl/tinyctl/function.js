// The faaas project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

'use strict';

const fs = require('fs');

class Function {
  constructor(name, modulePath, version) {
    this.name = name;
    this.modulePath = modulePath;
    this.version = version;
  }

  dig() {
    return new Promise((resolve, reject) => {
      fs.readFile(this.modulePath, 'utf8', (err, content) => {
        if (err) { reject(`Can't find module ${err}`); }
        this.moduleContent = content;
        resolve()
      })
    })
  }
}

module.exports = Function;