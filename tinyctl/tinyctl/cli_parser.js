// The microcall project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

'use strict';

const program = require('commander');

const launchCLI = () => {
  program
    .command('apply')
    .description('Apply an function into serverless backend.')
    .version('0.1.0')
    .parse(process.argv)

  let modulePath = process.argv[process.argv.length - 1];
  let name = modulePath.replace('.js', '');
  return { name, version: 'v1', module: modulePath }
}

module.exports = launchCLI;