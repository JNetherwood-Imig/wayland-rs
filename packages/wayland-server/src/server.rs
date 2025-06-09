use std::{io::Result, os::unix::net::UnixListener};

pub struct Server {
    socket: UnixListener,
}

impl Server {
    pub fn new() -> Result<Self> {
        Ok(Self {
            socket: UnixListener::bind("/run/user/1000/wayland-0")?,
        })
    }
}
