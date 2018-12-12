// The microcall project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

'use strict';

const Function = require('./tinyctl/function');
const launchCLI = require('./tinyctl/cli_parser');
const deploy = require('./tinyctl/request_handler');

let commandObject = launchCLI();

let locale = new Function(commandObject.name, commandObject.module, commandObject.version);
locale.dig()
  .then(() => {
    deploy('http://127.0.0.1:3000/deploy', locale)
      .then((body) => console.log(body))
      .catch((err) => console.error(err));
  })
  .catch((err) => console.error(err));