// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

'use strict';

const express = require('express');
const app = express();
const bodyParser = require('body-parser');
const runtimeFsPath = "../../tmp/node-js-v8";

const functionInstances = new Map();
const jsonParser = bodyParser.json();

function reflect(id) {
  // Return the passing module
  return require(`${runtimeFsPath}/${id}/index`);
}

app.get('/', (req, res) => {
  res.send('Hello world!');
})

app.get('/invoke', jsonParser, (req, res) => {
  if (req.body) {
    if (req.body.id) {
      let function_ = functionInstances.get(req.body.id);
      if (function_) {
        let param = null;
        try {
          param = JSON.parse(req.body.param);
        } catch (_) {
          param = {}
        }
        res.send(JSON.stringify(function_(param)));
      } else {
        res.send('The desired to be invoked function is not existed');
      }
    } else {
      // Bad request!
      res.sendStatus(400);
    }
  } else {
    // Bad request!
    res.sendStatus(400);
  }
})

app.post('/deploy', jsonParser, (req, res) => {
  // Pre-load function to avoid cold start.
  // The request object should contain uuid of function instance.
  if (req.body) {
    if (req.body.id) {
      functionInstances.set(req.body.id, reflect(req.body.id));
      console.log(`Successfully deploy function : ${req.body.id}`);
      res.send(`Successfully deploy! Endpoint at http://localhost:3000/endpoint/${req.body.id}`);
    } else {
      // Bad request!
      res.sendStatus(400);
    }
  } else {
    // Bad request!
    res.sendStatus(400);
  }
})

app.listen(5999);
