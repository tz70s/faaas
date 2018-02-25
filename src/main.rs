// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

extern crate tokio_uds;
extern crate tokio_core;
extern crate futures;
extern crate hyper;
#[macro_use]
extern crate serde_json;
extern crate uuid;

mod controller;
mod uds_handler;
mod runtime_fs;

fn main() {
    println!("Launch a tiny-invoker!");

    // Mount runtime fs
    runtime_fs::clean_up();
    runtime_fs::create_runtime_fs();
    runtime_fs::mount_nodejs_v8();
    controller::launch("127.0.0.1:3000");
}
