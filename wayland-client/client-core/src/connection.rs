use std::os::fd::{FromRawFd, OwnedFd, RawFd};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::result::Result;

use crate::proxy;

pub struct Connection {
    stream: UnixStream,
}

enum InternalEvent<E: Event> {
    DisplayError,
    DisplayDeleteId,
    Application(E),
}

impl<E: Event> Event for InternalEvent<E> {
    fn from_bytes(bytes: &[u8]) -> Self {
        if bytes[0] == 0 {
            Self::DisplayError
        } else if bytes[0] == 1 {
            Self::DisplayDeleteId
        } else {
            Self::Application(E::from_bytes(bytes))
        }
    }
}

impl Connection {
    pub fn get_display<I: proxy::Interface, const V: u32>(&self) -> proxy::Proxy<I, V> {
        proxy::Proxy::new(1)
    }

    pub fn get_next_event<E: Event>(&self) -> Result<Option<E>, std::io::Error> {
        todo!()
    }

    pub fn wait_next_event<E: Event>(&self) -> Result<E, std::io::Error> {
        todo!()
    }

    pub fn allocate_id(&self) -> u32 {
        0
    }

    pub fn default() -> Result<Self, ConnectError> {
        if let Ok(wayland_socket) = std::env::var("WAYLAND_SOCKET") {
            return Ok(Self::from_fd(unsafe {
                OwnedFd::from_raw_fd(wayland_socket.parse::<RawFd>().unwrap())
            }));
        }

        Self::from_path(std::env::var("WAYLAND_DISPLAY").unwrap_or("wayland-0".to_string()))
    }

    pub fn from_fd(fd: OwnedFd) -> Self {
        eprintln!("Using fd {:?}", fd);
        Self {
            stream: UnixStream::from(fd),
        }
    }

    pub fn from_path(path: String) -> Result<Self, ConnectError> {
        let path = if PathBuf::from(&path).is_absolute() {
            PathBuf::from(path)
        } else {
            let xdg_runtime_dir =
                std::env::var("XDG_RUNTIME_DIR").map_err(|_| ConnectError::NoXdgRuntimeDir)?;
            PathBuf::from(xdg_runtime_dir).join(path)
        };

        let stream =
            UnixStream::connect(&path).map_err(|_| ConnectError::InvalidDisplayPath(path))?;

        Ok(Self { stream })
    }
}

pub trait Event: Sized {
    fn from_bytes(bytes: &[u8]) -> Self;
}

#[derive(Debug)]
pub enum ConnectError {
    InvalidDisplayPath(PathBuf),
    NoXdgRuntimeDir,
}
