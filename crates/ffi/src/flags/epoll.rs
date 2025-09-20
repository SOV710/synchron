#![allow(non_camel_case_types)]
/// Flags to be passed to epoll_create1.
pub const EPOLL_CLOEXEC: i32 = 0o2000000;

/// Valid opcodes ("op" parameter) for epoll_ctl()
pub const EPOLL_CTL_ADD: i32 = 1; // Add a file descriptor
pub const EPOLL_CTL_DEL: i32 = 2; // Remove a file descriptor
pub const EPOLL_CTL_MOD: i32 = 3; // Modify a file descriptor

// epoll event flags
pub const EPOLLIN: i32 = 0x001;
pub const EPOLLPRI: i32 = 0x002;
pub const EPOLLOUT: i32 = 0x004;
pub const EPOLLERR: i32 = 0x008;
pub const EPOLLHUP: i32 = 0x010;
pub const EPOLLRDNORM: i32 = 0x040;
pub const EPOLLRDBAND: i32 = 0x080;
pub const EPOLLWRNORM: i32 = 0x100;
pub const EPOLLWRBAND: i32 = 0x200;
pub const EPOLLMSG: i32 = 0x400;
pub const EPOLLRDHUP: i32 = 0x2000;
pub const EPOLLEXCLUSIVE: i32 = 1 << 28;
pub const EPOLLWAKEUP: i32 = 1 << 29;
pub const EPOLLONESHOT: i32 = 1 << 30;
pub const EPOLLET: i32 = 1 << 31;

#[repr(C)]
pub struct epoll_event {
    pub events: u32,
    pub epoll_data_t: u64, // 与 union epoll_data_t 对齐（最通用）
}
