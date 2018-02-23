// The tiny-invoker project is under MIT License.
// Copyright (c) 2018 Tzu-Chiao Yeh

//! Uds handler for handling bi-directional connections with function instances.

use tokio_core::reactor::{Core, Handle};
use tokio_uds::UnixListener;
use tokio_core::io::read_to_end;
use futures::{Future, stream, Stream};
use std::str;
use std::fs;

pub struct UdsHandler<'a> {
    sock_name: &'static str,
    core: &'a mut Core,
}

impl<'a> UdsHandler<'a> {
    /// A new uds handler, run in a same event loop.
    pub fn new(sock_name: &'static str, core: &'a mut Core) -> Self {

        UdsHandler { sock_name, core }
    }

    pub fn launch(&mut self) {

        let handle = self.core.handle();

        // Bind listener.
        let listener = match UnixListener::bind(self.sock_name, &handle) {
            Ok(l) => l,
            Err(_) => {
                fs::remove_file(self.sock_name).unwrap();
                UnixListener::bind(self.sock_name, &handle).unwrap()
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

        self.core.run(task).unwrap();
    }
}
