// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

extern crate tokio_uds;
extern crate tokio_core;
extern crate futures;

mod uds_handler;

use tokio_core::reactor::Core;

fn main() {
    println!("Hello world!");

    let mut core = Core::new().unwrap();

    static SOCKET_PATH: &'static str = "invoker.sock";

    let mut u_handler = uds_handler::UdsHandler::new(SOCKET_PATH, &mut core);

    u_handler.launch();
}
