// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

extern crate tokio_uds;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
extern crate serde_json;

mod controller;
mod uds_handler;

use tokio_core::reactor::Core;

fn main() {
    println!("Launch a tiny-invoker!");
    controller::launch("127.0.0.1:3000");
}
