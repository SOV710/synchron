#![cfg(target_os = "linux")]

pub(crate) mod flags;
mod raw;
pub(crate) mod types;

pub mod error;

pub mod epoll;
pub mod fanotify;

pub mod uid;

pub use epoll::*;
pub use error::{Errno, Error};
pub use fanotify::*;
pub use raw::{read, write};
pub use uid::effective;
