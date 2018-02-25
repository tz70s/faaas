// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service};
use hyper::{StatusCode, Body, Chunk, Client, Uri, Get, Post, Request, Response};
use hyper;
use futures;
use futures::{Stream, Future};
use serde_json;
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use runtime_fs;
use tokio_core;

#[derive(Debug)]
struct FunctionMeta {
    name: String
}

struct Controller {
    client: hyper::Client<hyper::client::HttpConnector, Body>,
    function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
    uri: Uri,
}

impl Controller {
    fn new(client: hyper::Client<hyper::client::HttpConnector, Body>,
           function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>) -> Self {
        Controller {
            client,
            function_metas,
            uri: "http://127.0.0.1:5999/".parse().unwrap()
        }
    }
}

use futures::future::FutureResult;

fn get_root() -> Box<FutureResult<Response, hyper::Error>> {
    Box::new(futures::future::ok(Response::new()
        .with_status(StatusCode::Ok)
        .with_body("Welcome to the tiny invoker project")))
}

fn get_all(function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>)
    -> Box<FutureResult<Response, hyper::Error>> {

    let function_meta_mut_ref = function_metas.read().unwrap();
    let mut format_string = String::new();
    for (id, meta) in function_meta_mut_ref.iter() {
        format_string += format!("{:?}, {:?}\n", *id, *meta).as_str();
    }
    Box::new(futures::future::ok(Response::new()
        .with_status(StatusCode::Ok)
        .with_body(format_string)))
}

fn post_deploy(req: Request,
               function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
               client: hyper::Client<hyper::client::HttpConnector, Body>,
               uri: Uri)
    -> Box<Future<Item=Response, Error=hyper::Error>> {

    Box::new(req.body().concat2().map(move|buf| {
        let json: serde_json::Value = if let Ok(obj) = serde_json::from_slice(buf.as_ref()) {
            obj
        } else {
            // FIXME: Need to figure out a better error propagation.
            json!({})
        };

        // Generate function signature.
        let signature = Uuid::new_v4();
        let mut function_meta_mut_ref = function_metas.write().unwrap();
        function_meta_mut_ref.insert(signature.clone(),
                                     FunctionMeta {
                                         name: format!("{}", json["name"])
                                     });

        // FIXME: do not unwrap here.
        runtime_fs::mount_language_codes(&signature, json["moduleContent"].as_str().unwrap());

        // Request to language backend for triggering reflection.
        let mut req_to_invoke = Request::new(Post, "http://127.0.0.1:5999/deploy".parse().unwrap());
        let runtime_signal = json!({ "id": signature.to_string() }).to_string();
        req_to_invoke.headers_mut().set(ContentType::json());
        req_to_invoke.headers_mut().set(ContentLength(runtime_signal.len() as u64));
        req_to_invoke.set_body(runtime_signal);
        client.request(req_to_invoke)

    }).and_then(|res| res )
        .and_then(|res| futures::future::ok(res))
        .or_else(|_| {
            let mut response = Response::new();
            response.set_status(StatusCode::NotFound);
            response.set_body("Error occurred in deploy");
            futures::future::ok(response)
        }))
}

fn get_endpoint(req: Request, client: &hyper::Client<hyper::client::HttpConnector, Body>)
    -> Box<Future<Item=Response, Error=hyper::Error>> {

    // FIXME: CRITICAL POINT HERE, EXTREMELY SLOW NOW!
    let id = &req.path()[10..];
    let mut req_to_invoke = Request::new(Get, "http://127.0.0.1:5999/invoke".parse().unwrap());
    // FIXME: Hard coded param.
    let runtime_signal = json!({ "id": id, "param": "{\"name\": \"Jon\"}"}).to_string();
    req_to_invoke.headers_mut().set(ContentType::json());
    req_to_invoke.headers_mut().set(ContentLength(runtime_signal.len() as u64));
    req_to_invoke.set_body(runtime_signal);
    let invoke_future = client.request(req_to_invoke);
    Box::new(invoke_future.and_then(|res| { futures::future::ok(res) }))
}


fn bad_request() -> Box<FutureResult<Response, hyper::Error>> {
    let mut response = Response::new();
    response.set_status(StatusCode::NotFound);
    Box::new(futures::future::ok(response))
}

impl Service for Controller {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        match (req.method(), req.path()) {

            // Root path, say welcome message, remains for performance test.
            (&Get, "/") => get_root(),

            // Get all existed function instances.
            (&Get, "/all") => get_all(self.function_metas.clone()),

            // Deploy a function instance.
            (&Post, "/deploy") => post_deploy(req,
                                              self.function_metas.clone(),
                                              self.client.clone(),
                                              self.uri.clone()),

            // Invoke an endpoint/function
            _ if &req.path()[0..10] == "/endpoint/" => get_endpoint(req, &self.client),

            // 400
            _ => bad_request()
        }
    }
}

pub fn launch(addr_str: &str) {
    let addr = addr_str.parse().unwrap();
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client_handle = core.handle();
    let client = Client::configure().build(&client_handle);
    let function_metas = RwLock::new(HashMap::new());
    let arc = Arc::new(function_metas);

    let serve = Http::new()
        .serve_addr_handle(&addr, &handle, move || Ok(Controller::new(client.clone(), arc.clone()))).unwrap();
    println!("Listening on http://{} with 1 thread.", serve.incoming_ref().local_addr());

    let h2 = handle.clone();
    handle.spawn(serve.for_each(move |conn| {
        h2.clone().spawn(conn.map(|_| ()).map_err(|_|()));
        Ok(())
    }).map_err(|_| ()));

    core.run(futures::future::empty::<(), ()>()).unwrap();
}