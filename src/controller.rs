// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode, Body, Chunk};
use futures;
use futures::{Stream, Future};
use serde_json;

struct Controller {
    welcome: &'static str
}

impl Controller {
    fn new() -> Self {
        Controller { welcome: "Welcome to the tiny invoker!" }
    }
}

impl Service for Controller {
    type Request = Request;
    type Response = Response;
    type Error = ::hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            // Root path, say hello
            (&Method::Get, "/") => {
                response
                    .set_body(self.welcome);
            },
            (&Method::Get, "/invoke") => {
                response
                    .set_body("Invocation");
            },
            (&Method::Post, "/deploy") => {
                return Box::new(req.body().concat2().map(|ch| {
                    let json: serde_json::Value = if let Ok(obj) = serde_json::from_slice(ch.as_ref()) {
                        obj
                    } else {
                        response.set_status(StatusCode::BadRequest);
                        return response.with_body("Bad request");
                    };
                    response.with_body(format!("Deploy {}/{} at ...", json["name"], json["version"]))
                }));
            },
            _ => {
                // 404
                response.set_status(StatusCode::NotFound)
            }
        };

        Box::new(futures::future::ok(response))
    }
}

pub fn launch(addr_str: &str) {
    let addr = addr_str.parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Controller::new())).unwrap();
    server.run().unwrap();
}