// The faaas project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

extern crate futures;
extern crate futures_cpupool;
extern crate hyper;
#[macro_use]
extern crate serde_json;
extern crate tokio_core;
extern crate tokio_uds;
extern crate uuid;

mod controller;
mod action_fs;
mod action;
mod config;

fn main() {
    // Mount runtime fs
    action_fs::clean_up();
    action_fs::create_runtime_fs();
    action_fs::mount_nodejs_v8();
    controller::launch("127.0.0.1:3000");
}
