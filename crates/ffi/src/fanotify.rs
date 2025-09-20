use crate::flags::{fanotify as ff, fcntl as fc};
use crate::raw;
use crate::types::*;
use std::ffi::CString;
use std::io;

/// Wrapper for fanotify_init(2).
///
/// # Safety
/// You must ensure `flags` and `event_f_flags` are valid. The returned fd must be closed.
pub fn init(flags: u32, event_f_flags: i32) -> io::Result<RawFd> {
    let fd = unsafe { raw::fanotify_init(flags as c_int, event_f_flags as c_int) };
    if fd < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(fd)
    }
}

/// Wrapper for fanotify_mark(2).
///
/// - `fan_fd`: the fd returned by `init`.
/// - `mark_flags`: e.g., FAN_MARK_ADD | FAN_MARK_ONLYDIR | ...
/// - `mask`: e.g., FAN_MODIFY | FAN_OPEN | ...
/// - `dirfd`: use `flags::fcntl::AT_FDCWD` for relative `pathname` resolution.
/// - `pathname`: optional; pass `None` for marks on mounts/filesystems.
pub fn mark(
    fan_fd: RawFd,
    mark_flags: u32,
    mask: u64,
    dirfd: RawFd,
    pathname: Option<&str>,
) -> io::Result<()> {
    let c_path =
        match pathname {
            Some(p) => Some(CString::new(p).map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidInput, "pathname contains NUL")
            })?),
            None => None,
        };
    let ptr = c_path
        .as_ref()
        .map(|s| s.as_ptr())
        .unwrap_or(core::ptr::null());

    let rc = unsafe {
        raw::fanotify_mark(
            fan_fd as c_int,
            mark_flags as c_uint,
            mask as u64,
            dirfd as c_int,
            ptr,
        )
    };
    if rc < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Helper: common "content class, nonblocking, close-on-exec"
/// Fill the real flag values in `flags::fanotify` and `flags::fcntl` first.
pub fn init_default_nonblock() -> io::Result<RawFd> {
    let flags = ff::FAN_CLASS_CONTENT | ff::FAN_NONBLOCK | ff::FAN_CLOEXEC | ff::FAN_REPORT_PIDFD;
    let event_f_flags = fc::O_RDONLY | fc::O_LARGEFILE | fc::O_CLOEXEC;
    init(flags, event_f_flags)
}
