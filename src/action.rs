// The faaas project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

use hyper::header::{ContentLength, ContentType};
use hyper::{Body, Client, Get, Post, Request, Response, StatusCode};
use hyper;
use tokio_core::reactor::Handle;
use futures;
use futures::{Future, Stream};
use serde_json;
use std::collections::HashMap;
use uuid::Uuid;
use std::sync::{Arc, RwLock};
use action_fs;
use controller::FunctionMeta;
use tokio_uds::UnixStream;
// FIXME: This api will be a breaking change to be removed.
use tokio_core::io::write_all;
use tokio_core::io::read_to_end;
use config::{DEPLOY_URL, INVOKE_URL, UDS_SOCKET_PATH};

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
    req: Request,
    function_metas: Arc<RwLock<HashMap<Uuid, FunctionMeta>>>,
    client: Client<hyper::client::HttpConnector, Body>,
) -> Box<Future<Item = Response, Error = hyper::Error>> {
    Box::new(
        req.body()
            .concat2()
            .map(move |buf| {
                let json: serde_json::Value = if let Ok(obj) = serde_json::from_slice(buf.as_ref())
                {
                    obj
                } else {
                    // FIXME: Need to figure out a better error propagation.
                    json!({})
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
                action_fs::mount_language_codes(
                    &signature,
                    json["moduleContent"].as_str().unwrap(),
                );

                // Request to language backend for triggering reflection.
                let runtime_signal = json!({ "id": signature.to_string() }).to_string();
                let request = action_http_request(Post, DEPLOY_URL, runtime_signal);
                client.request(request)
            })
            .and_then(|res| res)
            .and_then(|res| futures::future::ok(res))
            .or_else(|_| {
                let mut response = Response::new();
                response.set_status(StatusCode::NotFound);
                response.set_body("Error occurred in deploy");
                futures::future::ok(response)
            }),
    )
}

pub fn get_endpoint(
    req: Request,
    client: &Client<hyper::client::HttpConnector, Body>,
) -> Box<Future<Item = Response, Error = hyper::Error>> {
    // FIXME: CRITICAL POINT HERE, EXTREMELY SLOW NOW!
    let id = &req.path()[10..];
    // FIXME: Hard coded param.
    let runtime_signal = json!({ "id": id, "param": "{\"name\": \"Jon\"}"}).to_string();
    let request = action_http_request(Get, INVOKE_URL, runtime_signal);
    Box::new(client.request(request))
}

pub fn get_uds_endpoint(
    req: Request,
    handle: Handle,
) -> Box<Future<Item = Response, Error = hyper::Error>> {
    // FIXME: CRITICAL POINT HERE, EXTREMELY SLOW NOW!
    let id = &req.path()[14..];
    // FIXME: Hard coded param.
    let runtime_signal = json!({ "id": id, "param": "{\"name\": \"Jon\"}"});
    let runtime_signal = Vec::<u8>::from(runtime_signal.to_string());

    let uds_stream = if let Ok(stream) = UnixStream::connect(UDS_SOCKET_PATH, &handle) {
        stream
    } else {
        return Box::new(futures::future::ok(
            Response::new().with_status(StatusCode::BadRequest),
        ));
    };

    Box::new(
        write_all(uds_stream, runtime_signal)
            .and_then(|(uds_stream, mut buf)| {
                buf.clear();
                read_to_end(uds_stream, buf)
            })
            .and_then(|(uds_stream, buf)| {
                futures::future::ok(Response::new().with_status(StatusCode::Ok).with_body(buf))
            })
            .or_else(|_| {
                let mut response = Response::new();
                response.set_status(StatusCode::BadRequest);
                futures::future::ok(response)
            }),
    )
}
