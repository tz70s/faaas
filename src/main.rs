extern crate tokio_uds;
extern crate tokio_core;
extern crate futures;

use tokio_core::io::read_to_end;
use tokio_core::reactor::Core;
use tokio_uds::UnixListener;
use futures::{Future, Stream};
use std::str;
use std::fs;

fn main() {
    println!("Hello world!");

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    static SOCKET_PATH: &'static str = "invoker.sock";

    let listener = match UnixListener::bind(SOCKET_PATH, &handle) {
        Ok(l) => l,
        Err(_) => {
            fs::remove_file(SOCKET_PATH).unwrap();
            UnixListener::bind(SOCKET_PATH, &handle).unwrap()
        }
    };

    let task = listener.incoming().for_each(|(socket, addr)| {
        let buf = Vec::new();
        let reader = read_to_end(socket, buf).map(move|(_, _buf)| {
            println!("Incoming from : {:?} => {:?}", addr, str::from_utf8(&_buf).unwrap());
        }).then(|_| Ok(()));
        handle.spawn(reader);
        Ok(())
    });

    core.run(task).unwrap();
}
