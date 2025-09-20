use crate::error::{retry_eintr, Errno, Error, Result};
use crate::flags::{fanotify as fflag, fcntl as fcntl_flag};
use crate::raw;
use crate::types::*;
use std::ffi::CString;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd, RawFd};

/// ---------- Strong-typed flag ----------

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FanotifyInitFlags(pub u32);

impl FanotifyInitFlags {
    pub const EMPTY: Self = Self(0);
    // Hints (fill values in flags.rs):
    pub const CLOEXEC: Self = Self(fflag::FAN_CLOEXEC);
    pub const NONBLOCK: Self = Self(fflag::FAN_NONBLOCK);
    pub const CLASS_CONTENT: Self = Self(fflag::FAN_CLASS_CONTENT);
    pub const CLASS_NOTIF: Self = Self(fflag::FAN_CLASS_NOTIF);
    pub const CLASS_PRE_CONTENT: Self = Self(fflag::FAN_CLASS_PRE_CONTENT);
    pub const REPORT_PIDFD: Self = Self(fflag::FAN_REPORT_PIDFD);
    pub const REPORT_FID: Self = Self(fflag::FAN_REPORT_FID);
    pub const REPORT_DIR_FID: Self = Self(fflag::FAN_REPORT_DIR_FID);
    pub const REPORT_TID: Self = Self(fflag::FAN_REPORT_TID);
    pub const REPORT_DFID_NAME: Self = Self(fflag::FAN_REPORT_DFID_NAME);
    pub const UNLIMITED_QUEUE: Self = Self(fflag::FAN_UNLIMITED_QUEUE);
    pub const UNLIMITED_MARKS: Self = Self(fflag::FAN_UNLIMITED_MARKS);
}
impl core::ops::BitOr for FanotifyInitFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl core::ops::BitOrAssign for FanotifyInitFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OpenFlags(pub i32);
impl OpenFlags {
    pub const EMPTY: Self = Self(0);
    pub const RDONLY: Self = Self(fcntl_flag::O_RDONLY);
    pub const WRONLY: Self = Self(fcntl_flag::O_WRONLY);
    pub const RDWR: Self = Self(fcntl_flag::O_RDWR);
    pub const CLOEXEC: Self = Self(fcntl_flag::O_CLOEXEC);
    pub const NONBLOCK: Self = Self(fcntl_flag::O_NONBLOCK);
    pub const LARGEFILE: Self = Self(fcntl_flag::O_LARGEFILE);
}
impl core::ops::BitOr for OpenFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl core::ops::BitOrAssign for OpenFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FanotifyMarkFlags(pub u32);
impl FanotifyMarkFlags {
    pub const EMPTY: Self = Self(0);
    pub const ADD: Self = Self(fflag::FAN_MARK_ADD);
    pub const REMOVE: Self = Self(fflag::FAN_MARK_REMOVE);
    pub const ONLYDIR: Self = Self(fflag::FAN_MARK_ONLYDIR);
    pub const INODE: Self = Self(fflag::FAN_MARK_INODE);
    pub const MOUNT: Self = Self(fflag::FAN_MARK_MOUNT);
    pub const FILESYSTEM: Self = Self(fflag::FAN_MARK_FILESYSTEM);
    pub const DONT_FOLLOW: Self = Self(fflag::FAN_MARK_DONT_FOLLOW);
    pub const IGNORED_MASK: Self = Self(fflag::FAN_MARK_IGNORED_MASK);
    pub const IGNORED_SURV_MODIFY: Self = Self(fflag::FAN_MARK_IGNORED_SURV_MODIFY);
    pub const EVICTABLE: Self = Self(fflag::FAN_MARK_EVICTABLE);
}
impl core::ops::BitOr for FanotifyMarkFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl core::ops::BitOrAssign for FanotifyMarkFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FanotifyEventMask(pub u64);
impl FanotifyEventMask {
    pub const EMPTY: Self = Self(0);
    // Hints (fill in flags.rs):
    pub const ACCESS: Self = Self(fflag::FAN_ACCESS);
    pub const MODIFY: Self = Self(fflag::FAN_MODIFY);
    pub const OPEN: Self = Self(fflag::FAN_OPEN);
    pub const OPEN_EXEC: Self = Self(fflag::FAN_OPEN_EXEC);
    pub const ATTRIB: Self = Self(fflag::FAN_ATTRIB);
    pub const CREATE: Self = Self(fflag::FAN_CREATE);
    pub const DELETE: Self = Self(fflag::FAN_DELETE);
    pub const DELETE_SELF: Self = Self(fflag::FAN_DELETE_SELF);
    pub const MOVE_SELF: Self = Self(fflag::FAN_MOVE_SELF);
    pub const MOVED_FROM: Self = Self(fflag::FAN_MOVED_FROM);
    pub const MOVED_TO: Self = Self(fflag::FAN_MOVED_TO);
    pub const RENAME: Self = Self(fflag::FAN_RENAME);
    pub const EVENT_ON_CHILD: Self = Self(fflag::FAN_EVENT_ON_CHILD);
    pub const ONDIR: Self = Self(fflag::FAN_ONDIR);
    pub const CLOSE: Self = Self(fflag::FAN_CLOSE);
    pub const OPEN_PERM: Self = Self(fflag::FAN_OPEN_PERM);
    pub const ACCESS_PERM: Self = Self(fflag::FAN_ACCESS_PERM);
    pub const OPEN_EXEC_PERM: Self = Self(fflag::FAN_OPEN_EXEC_PERM);
    pub const Q_OVERFLOW: Self = Self(fflag::FAN_Q_OVERFLOW);
}
impl core::ops::BitOr for FanotifyEventMask {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl core::ops::BitOrAssign for FanotifyEventMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

pub struct FanotifyEvent {
    pub mask: FanotifyEventMask,
    pub pid: i32,
    /// 事件对象文件句柄；部分事件会给出一个临时 fd（需关闭）
    pub object: Option<OwnedFd>,
    /// 原始 metadata 长度（调试用）
    pub raw_len: u32,
}

/// Strong-typed directory fd (for pathname resolution).
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DirFd(pub RawFd);
impl DirFd {
    pub const CWD: Self = Self(fcntl_flag::AT_FDCWD);
}

/// RAII fanotify handle.
pub struct Fanotify {
    fd: OwnedFd,
}

impl Fanotify {
    /// Create a fanotify instance (RAII).
    pub fn new(flags: FanotifyInitFlags, event_f_flags: OpenFlags) -> Result<Self> {
        let fd = retry_eintr(|| unsafe {
            raw::fanotify_init(flags.0 as c_int, event_f_flags.0 as c_int)
        })?;
        // Safety: fd is a fresh, owned descriptor from the kernel.
        let owned = unsafe { OwnedFd::from_raw_fd(fd) };
        Ok(Self { fd: owned })
    }

    /// Add/Remove a mark. `pathname=None` for mount/filesystem marks.
    pub fn mark(
        &self,
        mark_flags: FanotifyMarkFlags,
        mask: FanotifyEventMask,
        dirfd: DirFd,
        pathname: Option<&str>,
    ) -> Result<()> {
        let c_path = pathname
            .map(|p| {
                CString::new(p).map_err(|_| Error {
                    errno: Errno::EINVAL,
                })
            })
            .transpose()?;
        let ptr = c_path
            .as_ref()
            .map(|s| s.as_ptr())
            .unwrap_or(core::ptr::null());

        retry_eintr(|| unsafe {
            raw::fanotify_mark(
                self.fd.as_raw_fd() as c_int,
                mark_flags.0 as c_uint,
                mask.0 as u64,
                dirfd.0 as c_int,
                ptr,
            )
        })?;

        Ok(())
    }

    /// 从 fanotify 实例 fd 读取事件并解析。
    /// - `buf`：可写缓冲（如 8KB/64KB）；返回解析出的事件列表（每个自带 RAII fd）。
    /// - EAGAIN 时返回 Ok(vec![])。
    pub fn read_events(&self, buf: &mut [u8]) -> Result<Vec<FanotifyEvent>> {
        // 手动处理 EAGAIN 语义；EINTR 交给 retry_eintr。
        let rc = unsafe {
            raw::read(
                self.fd.as_raw_fd() as c_int,
                buf.as_mut_ptr() as *mut _,
                buf.len(),
            )
        };

        if rc < 0 {
            let ioerr = std::io::Error::last_os_error();
            // WouldBlock / EAGAIN -> 空集合（保持你原有行为）
            if ioerr.raw_os_error() == Some(crate::flags::errno::EAGAIN)
                || ioerr.kind() == std::io::ErrorKind::WouldBlock
            {
                return Ok(vec![]);
            }
            // 其它错误转成 Error（包含可匹配的 Errno）
            return Err(Error::from(ioerr));
        }

        let n = rc as usize;
        if n == 0 {
            // 读到 0：暂无可读事件（极少见），与 EAGAIN 一致地返回空
            return Ok(vec![]);
        }

        // 解析环形缓冲里的多条事件（这里的 parse_* 应改为返回 Result<_, Error>）
        unsafe { parse_events_from_filled(buf, n) }
    }

    pub fn respond_permission(&self, event_fd: &OwnedFd, allow: bool) -> Result<()> {
        let resp = fanotify_response {
            fd: event_fd.as_raw_fd(),
            response: if allow {
                fflag::FAN_ALLOW
            } else {
                fflag::FAN_DENY
            },
        };

        // 写 fanotify 响应：自动重试 EINTR；其它 errno 返回 Error
        retry_eintr(|| unsafe {
            raw::write(
                self.fd.as_raw_fd() as c_int,
                &resp as *const _ as *const _,
                core::mem::size_of::<fanotify_response>(),
            )
        })?;

        Ok(())
    }
}

unsafe fn parse_events_from_filled(buf: &mut [u8], filled: usize) -> Result<Vec<FanotifyEvent>> {
    let mut out = Vec::new();
    let mut off = 0usize;

    while off + core::mem::size_of::<fanotify_event_metadata>() <= filled {
        let meta_ptr = buf.as_ptr().add(off) as *const fanotify_event_metadata;
        let meta = &*meta_ptr;

        // 基本一致性检查
        if meta.vers != fflag::FANOTIFY_METADATA_VERSION && fflag::FANOTIFY_METADATA_VERSION != 0 {
            return Err(crate::error::Error::invalid_data());
        }

        let evlen = meta.event_len as usize;
        if evlen == 0 || off + evlen > filled {
            return Err(crate::error::Error::truncated());
        }

        // 取出 mask/pid/fd
        let mask = FanotifyEventMask(meta.mask);
        let pid = meta.pid;
        let raw_fd = meta.fd;

        // 注意：内核把对象 fd 放在 metadata 头里；它是“需要你关闭”的临时 fd。
        // 用 OwnedFd 接住，交给 RAII。
        let obj_fd = if raw_fd >= 0 {
            // SAFETY: 事件拥有这个 fd 的所有权；把它转成 OwnedFd 以便 Drop 关闭。
            Some(OwnedFd::from_raw_fd(raw_fd))
        } else {
            None
        };

        out.push(FanotifyEvent {
            mask,
            pid,
            object: obj_fd,
            raw_len: meta.event_len,
        });

        // 下一个事件
        off += evlen;
    }

    Ok(out)
}

impl AsFd for Fanotify {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}
