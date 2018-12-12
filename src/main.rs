// The microcall project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_uds;
extern crate uuid;

mod action;
mod action_container;
mod constants;
mod controller;

/// Mount node.js runtime for invocation.
/// It'll clean up existed file directory, create a new one and mount it.
fn mount_node_runtime() {
    // Create new runtime file directory.
    action_container::create_runtime_container();
    // Mount node.js runtime.
    action_container::mount_nodejs_v8();
}

fn main() {
    env_logger::init();
    info!("Starting up microcall runtime.");
    mount_node_runtime();
    controller::launch("127.0.0.1:3000");
}
