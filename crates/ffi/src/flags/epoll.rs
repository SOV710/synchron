#![allow(non_camel_case_types)]
/// Flags to be passed to epoll_create1.
pub const EPOLL_CLOEXEC: i32 = 0o2000000;

/// Valid opcodes ("op" parameter) for epoll_ctl()
pub const EPOLL_CTL_ADD: i32 = 1; // Add a file descriptor
pub const EPOLL_CTL_DEL: i32 = 2; // Remove a file descriptor
pub const EPOLL_CTL_MOD: i32 = 3; // Modify a file descriptor

// epoll event flags
pub const EPOLLIN: u32 = 0x001;
pub const EPOLLPRI: u32 = 0x002;
pub const EPOLLOUT: u32 = 0x004;
pub const EPOLLERR: u32 = 0x008;
pub const EPOLLHUP: u32 = 0x010;
// pub const EPOLLRDNORM: u32 = 0x040;
// pub const EPOLLRDBAND: u32 = 0x080;
// pub const EPOLLWRNORM: u32 = 0x100;
// pub const EPOLLWRBAND: u32 = 0x200;
// pub const EPOLLMSG: u32 = 0x400;
pub const EPOLLRDHUP: u32 = 0x2000;
// pub const EPOLLEXCLUSIVE: u32 = 1 << 28;
// pub const EPOLLWAKEUP: u32 = 1 << 29;
pub const EPOLLONESHOT: u32 = 1 << 30;
pub const EPOLLET: u32 = 1 << 31;
