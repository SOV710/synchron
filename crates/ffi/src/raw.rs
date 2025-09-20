use crate::types::*;
use core::ffi::c_char;

#[link(name = "c")]
extern "C" {
    // fanotify
    pub fn fanotify_init(flags: c_int, event_f_flags: c_int) -> c_int;

    pub fn fanotify_mark(
        fanotify_fd: c_int,
        flags: c_uint,
        mask: u64,
        dirfd: c_int,
        pathname: *const c_char,
    ) -> c_int;

    // epoll
    pub fn epoll_create1(flags: c_int) -> c_int;

    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut EpollEvent) -> c_int;

    pub fn epoll_wait(
        epfd: c_int,
        events: *mut EpollEvent,
        maxevents: c_int,
        timeout: c_int,
    ) -> c_int;

    pub fn epoll_pwait(
        epfd: c_int,
        events: *mut EpollEvent,
        maxevents: c_int,
        timeout: c_int,
        sigmask: *const SigSet,
    ) -> c_int;

    // glibc 2.35+: epoll_pwait2
    pub fn epoll_pwait2(
        epfd: c_int,
        events: *mut EpollEvent,
        maxevents: c_int,
        timeout: *const TimeSpec,
        sigmask: *const SigSet,
    ) -> c_int;
}
