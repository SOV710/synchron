#![cfg(target_os = "linux")]

pub mod flags;
mod raw;
pub mod types;

pub mod epoll;
pub mod fanotify;

pub use flags::*;
pub use types::*;
