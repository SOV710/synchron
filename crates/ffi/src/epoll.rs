use crate::flags::epoll as ef;
use crate::raw;
use crate::types::*;
use std::io;

/// Create epoll instance (epoll_create1).
/// Caller must close the returned fd.
pub fn create(flags: i32) -> io::Result<RawFd> {
    let fd = unsafe { raw::epoll_create1(flags as c_int) };
    if fd < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(fd)
    }
}

/// epoll_ctl wrapper.
pub fn ctl(epfd: RawFd, op: i32, fd: RawFd, event: &mut EpollEvent) -> io::Result<()> {
    let rc = unsafe { raw::epoll_ctl(epfd as c_int, op as c_int, fd as c_int, event as *mut _) };
    if rc < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// epoll_wait wrapper: returns number of events or error.
pub fn wait(epfd: RawFd, events: &mut [EpollEvent], timeout_ms: i32) -> io::Result<usize> {
    let rc = unsafe {
        raw::epoll_wait(
            epfd as c_int,
            events.as_mut_ptr(),
            events.len() as c_int,
            timeout_ms as c_int,
        )
    };
    if rc < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(rc as usize)
    }
}

/// epoll_pwait wrapper with optional SigSet (pass None for no mask).
pub fn pwait(
    epfd: RawFd,
    events: &mut [EpollEvent],
    timeout_ms: i32,
    mask: Option<&SigSet>,
) -> io::Result<usize> {
    let rc = unsafe {
        raw::epoll_pwait(
            epfd as c_int,
            events.as_mut_ptr(),
            events.len() as c_int,
            timeout_ms as c_int,
            mask.map(|m| m as *const _).unwrap_or(core::ptr::null()),
        )
    };
    if rc < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(rc as usize)
    }
}

/// epoll_pwait2 wrapper with absolute timeout as `TimeSpec`.
pub fn pwait2(
    epfd: RawFd,
    events: &mut [EpollEvent],
    timeout: Option<&TimeSpec>,
    mask: Option<&SigSet>,
) -> io::Result<usize> {
    let rc = unsafe {
        raw::epoll_pwait2(
            epfd as c_int,
            events.as_mut_ptr(),
            events.len() as c_int,
            timeout.map(|t| t as *const _).unwrap_or(core::ptr::null()),
            mask.map(|m| m as *const _).unwrap_or(core::ptr::null()),
        )
    };
    if rc < 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(rc as usize)
    }
}

/// Example helper to build an `EpollEvent` with fd payload.
pub fn make_event(events_mask: u32, fd: RawFd) -> EpollEvent {
    EpollEvent {
        events: events_mask,
        data: EpollData { fd },
    }
}
