// The faaas project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

'use strict';

const net = require('net');
const fs = require('fs');
const express = require('express');
const app = express();
const bodyParser = require('body-parser');
const runtimeFsPath = "../../tmp/node-js-v8";
const functionInstances = new Map();
const jsonParser = bodyParser.json();

function createSock(path) {
  fs.stat(path, (err, stat) => {
    if (err) {
      // No left over socket.
      let server = new UdsServer(path);
      process.on('SIGINT', () => { server.destruct(); });
    } else {
      fs.unlink(path, (err) => {
        if (err) {
          console.error(err);
          process.exit(1);
        }
        let server = new UdsServer(path);
        process.on('SIGINT', () => { server.destruct(); });
      });
    }
  });
}

class UdsServer {
  constructor(socket) {
    this.socket = socket;
    this.count = 0;
    this.connections = [];
    this.server = net.createServer((stream) => {

      // On closed.
      stream.on('end', () => {
        // TODO: Clean up connections.
        stream.destroy();
      });

      // On data.
      stream.on('data', (msg) => {
        if (msg) {
          let obj = null;
          try {
            obj = JSON.parse(msg.toString());
            if (obj && obj.id) {
              let function_ = functionInstances.get(obj.id);
              let value = function_(JSON.parse(obj.param));
              stream.write(JSON.stringify(value), () => {
                stream.destroy();
              })
            }
          } catch (_) {
            stream.write('Bad request', () => {
              stream.destroy();
            });
          }
        } else {
          stream.write('Bad request', () => {
            stream.destroy();
          });
        }
      });

    }).listen(socket)
      .on('connection', (socket) => {
        // Do nothing.
      });
  }

  destruct() {
    fs.unlink(this.socket, (err) => {
      if (err) {
        console.error(err);
        process.exit(1);
      } else {
        process.exit(0);
      }
    });
  }
}

function reflect(id) {
  // Return the passing module
  return require(`${runtimeFsPath}/${id}/index`);
}

app.get('/', (req, res) => {
  res.send('Hello world!');
})

app.post('/deploy', jsonParser, (req, res) => {
  // Pre-load function to avoid cold start.
  // The request object should contain uuid of function instance.
  if (req.body) {
    if (req.body.id) {
      functionInstances.set(req.body.id, reflect(req.body.id));
      console.log(`Successfully deploy function : ${req.body.id}`);
      createSock('action.sock');
      res.send(`Successfully deploy! Uds Endpoint at : http://localhost:3000/uds-endpoint/${req.body.id}`);
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