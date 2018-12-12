// The microcall project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};

use futures;
use futures::{Future, Stream};
use hyper;
use hyper::{Body, Client, Get, Post, Request, Response, StatusCode};
use hyper::header::{ContentLength, ContentType};
use serde_json;
// FIXME: This api will be a breaking change to be removed.
use tokio_core::io::read_to_end;
use tokio_core::io::write_all;
use tokio_core::reactor::Handle;
use tokio_uds::UnixStream;
use uuid::Uuid;

use action_container;
use constants::{DEPLOYMENT_URL, INVOCATION_URL, UDS_SOCKET_PATH};
use controller::FunctionMeta;

fn action_http_request(method: hyper::Method, url: &str, content: String) -> Request {
    let mut request = Request::new(method, url.parse().unwrap());
    request.headers_mut().set(ContentType::json());
    request
        .headers_mut()
        .set(ContentLength(content.len() as u64));
    request.set_body(content);
    request
}

pub fn post_deploy(
    request: Request,
    function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
    client: Client<hyper::client::HttpConnector, Body>,
) -> Box<Future<Item=Response, Error=hyper::Error>> {
    let request_stream = request.body().concat2();
    let redirect_to_post = request_stream.map(move |buf| {
        let json: serde_json::Value = match serde_json::from_slice(buf.as_ref()) {
            Ok(json_object) => json_object,
            Err(_) => {
                debug!("Failed to serde incoming request from slice");
                json!({})
            }
        };

        // Generate function signature.
        let signature = Uuid::new_v4();
        let mut function_meta_mut_ref = function_metas.write().unwrap();
        function_meta_mut_ref.insert(
            signature.clone(),
            FunctionMeta {
                name: format!("{}", json["name"]),
            },
        );

        // FIXME: do not unwrap here.
        action_container::mount_language_codes(&signature, json["moduleContent"].as_str().unwrap());

        // Request to language backend for triggering reflection.
        let runtime_signal = json!({ "id": signature.to_string() }).to_string();
        let request = action_http_request(Post, DEPLOYMENT_URL, runtime_signal);
        client.request(request)
    });

    let result = redirect_to_post
        .and_then(|res| res)
        .and_then(|res| futures::future::ok(res))
        .or_else(|_| {
            let mut response = Response::new();
            response.set_status(StatusCode::NotFound);
            response.set_body("Error occurred in deploy");
            futures::future::ok(response)
        });

    Box::new(result)
}

pub fn get_endpoint(
    req: Request,
    client: &Client<hyper::client::HttpConnector, Body>,
) -> Box<Future<Item=Response, Error=hyper::Error>> {
    // FIXME: CRITICAL POINT HERE, EXTREMELY SLOW NOW!
    let id = &req.path()[10..];
    // FIXME: Hard coded param.
    let runtime_signal = json!({ "id": id, "param": "{\"name\": \"Jon\"}"}).to_string();
    let request = action_http_request(Get, INVOCATION_URL, runtime_signal);
    Box::new(client.request(request))
}

pub fn get_uds_endpoint(
    req: Request,
    handle: Handle,
) -> Box<Future<Item=Response, Error=hyper::Error>> {
    // FIXME: CRITICAL POINT HERE, EXTREMELY SLOW NOW!
    let id = &req.path()[14..];
    // FIXME: Hard coded param.
    let runtime_signal = json!({ "id": id, "param": "{\"name\": \"Jon\"}"});
    let runtime_signal = Vec::<u8>::from(runtime_signal.to_string());

    let uds_stream = match UnixStream::connect(UDS_SOCKET_PATH, &handle) {
        Ok(stream) => stream,
        Err(err) => {
            debug!("Failed to connected via uds socket {}", err.description());
            return Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::BadRequest),
            ));
        }
    };

    let write_result = write_all(uds_stream, runtime_signal)
        .and_then(|(uds_stream, mut buf)| {
            buf.clear();
            read_to_end(uds_stream, buf)
        })
        .and_then(|(_, buf)| {
            futures::future::ok(Response::new().with_status(StatusCode::Ok).with_body(buf))
        })
        .or_else(|_| {
            let mut response = Response::new();
            response.set_status(StatusCode::BadRequest);
            futures::future::ok(response)
        });

    Box::new(write_result)
}
