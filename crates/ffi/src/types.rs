#![allow(non_camel_case_types)]

use core::ffi::c_void;

pub type c_int = i32;
pub type c_uint = u32;
pub type size_t = usize;
pub type ssize_t = isize;
pub type RawFd = i32;

/// Union used by epoll_event.
#[repr(C)]
pub union EpollData {
    pub ptr: *mut c_void,
    pub fd: RawFd,
    pub u32_: u32,
    pub u64_: u64,
}

/// Kernel epoll event layout.
#[repr(C)]
pub struct EpollEvent {
    pub events: u32,
    pub data: EpollData,
}

/// Portable sigset_t replacement (glibc style).
/// glibc uses 1024 bits => 16 * usize (64-bit) or 32 * usize (32-bit).
#[cfg(target_pointer_width = "64")]
pub const SIGSET_NWORDS: usize = 16;
#[cfg(target_pointer_width = "32")]
pub const SIGSET_NWORDS: usize = 32;

#[repr(C)]
pub struct SigSet {
    pub __val: [usize; SIGSET_NWORDS],
}

impl SigSet {
    pub fn empty() -> Self {
        Self {
            __val: [0; SIGSET_NWORDS],
        }
    }
}

/// timespec for epoll_pwait2. On most 64-bit Linux, both i64.
#[cfg(any(
    target_pointer_width = "64",
    target_arch = "x86_64",
    target_arch = "aarch64"
))]
#[repr(C)]
pub struct TimeSpec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

/// A conservative 32-bit fallback (note: some ABIs use 64-bit time_t via different syscalls)
#[cfg(all(
    target_pointer_width = "32",
    not(any(target_arch = "x86_64", target_arch = "aarch64"))
))]
#[repr(C)]
pub struct TimeSpec {
    pub tv_sec: i32,
    pub tv_nsec: i32,
}

/// fanotify event metadata (stable, widely used)
/// See: include/uapi/linux/fanotify.h
#[repr(C)]
pub struct fanotify_event_metadata {
    pub event_len: u32,
    pub vers: u8,
    pub reserved: u8,
    pub metadata_len: u16,
    pub mask: u64,
    pub fd: i32,
    pub pid: i32,
}

/// fanotify response payload to allow or deny
#[repr(C)]
pub struct fanotify_response {
    pub fd: i32,
    pub response: u32,
}

/// Variable-length file_handle header used by FAN_REPORT_FID
/// (flexible array member f_handle[0] omitted)
#[repr(C)]
pub struct file_handle {
    pub handle_bytes: u32,
    pub handle_type: i32,
    // pub f_handle: [u8; 0], // flexible tail
}

/// Generic info header used by fanotify info records (e.g., FID)
#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: u8,
    pub pad: u8,
    pub len: u16,
}

/// FID info record header (points to file_handle payload)
#[repr(C)]
pub struct fanotify_event_info_fid {
    pub hdr: fanotify_event_info_header,
    pub fh: file_handle,
}
