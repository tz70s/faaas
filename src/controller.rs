// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode, Body, Chunk};
use hyper;
use futures;
use futures::{Stream, Future};
use serde_json;
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use runtime_fs;

#[derive(Debug)]
struct FunctionMeta {
    name: String
}

struct Controller {
    function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
}

impl Controller {
    fn new(function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>) -> Self {
        Controller {
            function_metas
        }
    }
}

impl Service for Controller {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            // Root path, say hello
            (&Method::Get, "/") => {
                response
                    .set_body("Welcome to the tiny invoker project");
            },
            (&Method::Get, "/all") => {
                let function_meta_mut_ref = self.function_metas.read().unwrap();
                let mut format_string = String::new();
                for (id, meta) in function_meta_mut_ref.iter() {
                    format_string += format!("{:?}, {:?}\n", *id, *meta).as_str();
                }
                response
                    .set_body(format_string)
            },
            (&Method::Get, "/invoke") => {
                response
                    .set_body("Invocation");
            },
            (&Method::Post, "/deploy") => {
                let function_meta_ref = self.function_metas.clone();
                return Box::new(req.body().concat2().map(move|ch| {
                    let json: serde_json::Value = if let Ok(obj) = serde_json::from_slice(ch.as_ref()) {
                        obj
                    } else {
                        response.set_status(StatusCode::BadRequest);
                        return response.with_body("Bad request");
                    };
                    let signature = Uuid::new_v4();
                    let mut function_meta_mut_ref = function_meta_ref.write().unwrap();
                    function_meta_mut_ref.insert(signature.clone(),
                                                 FunctionMeta {
                                                     name: format!("{}", json["name"])
                                                 });
                    runtime_fs::mount_language_codes(&signature, json["moduleContent"].as_str().unwrap());
                    response.with_body(format!("Deploy {}/{} at {}", json["name"], json["version"], signature))
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
    let function_metas = RwLock::new(HashMap::new());
    let arc = Arc::new(function_metas);
    let server = Http::new()
        .bind(&addr,move || Ok(Controller::new(arc.clone()))).unwrap();
    server.run().unwrap();
}