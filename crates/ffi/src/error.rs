use core::fmt;
use std::io;

use crate::flags::errno::{self as e};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Errno {
    EPERM,
    ENOENT,
    ESRCH,
    EINTR,
    EIO,
    ENXIO,
    E2BIG,
    ENOEXEC,
    EBADF,
    ECHILD,
    EAGAIN,
    ENOMEM,
    EACCES,
    EFAULT,
    ENOTBLK,
    EBUSY,
    EEXIST,
    EXDEV,
    ENODEV,
    ENOTDIR,
    EISDIR,
    EINVAL,
    ENFILE,
    EMFILE,
    ENOTTY,
    ETXTBSY,
    EFBIG,
    ENOSPC,
    ESPIPE,
    EROFS,
    EMLINK,
    EPIPE,
    EDOM,
    ERANGE,
    EPROTO,
    EOVERFLOW,
    Unknown(i32),
}

impl Errno {
    #[inline]
    pub fn from_raw(n: i32) -> Self {
        match n {
            x if x == e::EPERM => Errno::EPERM,
            x if x == e::ENOENT => Errno::ENOENT,
            x if x == e::ESRCH => Errno::ESRCH,
            x if x == e::EINTR => Errno::EINTR,
            x if x == e::EIO => Errno::EIO,
            x if x == e::ENXIO => Errno::ENXIO,
            x if x == e::E2BIG => Errno::E2BIG,
            x if x == e::ENOEXEC => Errno::ENOEXEC,
            x if x == e::EBADF => Errno::EBADF,
            x if x == e::ECHILD => Errno::ECHILD,
            x if x == e::EAGAIN => Errno::EAGAIN,
            x if x == e::ENOMEM => Errno::ENOMEM,
            x if x == e::EACCES => Errno::EACCES,
            x if x == e::EFAULT => Errno::EFAULT,
            x if x == e::ENOTBLK => Errno::ENOTBLK,
            x if x == e::EBUSY => Errno::EBUSY,
            x if x == e::EEXIST => Errno::EEXIST,
            x if x == e::EXDEV => Errno::EXDEV,
            x if x == e::ENODEV => Errno::ENODEV,
            x if x == e::ENOTDIR => Errno::ENOTDIR,
            x if x == e::EISDIR => Errno::EISDIR,
            x if x == e::EINVAL => Errno::EINVAL,
            x if x == e::ENFILE => Errno::ENFILE,
            x if x == e::EMFILE => Errno::EMFILE,
            x if x == e::ENOTTY => Errno::ENOTTY,
            x if x == e::ETXTBSY => Errno::ETXTBSY,
            x if x == e::EFBIG => Errno::EFBIG,
            x if x == e::ENOSPC => Errno::ENOSPC,
            x if x == e::ESPIPE => Errno::ESPIPE,
            x if x == e::EROFS => Errno::EROFS,
            x if x == e::EMLINK => Errno::EMLINK,
            x if x == e::EPIPE => Errno::EPIPE,
            x if x == e::EDOM => Errno::EDOM,
            x if x == e::ERANGE => Errno::ERANGE,
            x if x == e::EPROTO => Errno::EPROTO,
            x if x == e::EOVERFLOW => Errno::EOVERFLOW,
            other => Errno::Unknown(other),
        }
    }

    #[inline]
    pub fn to_raw(self) -> i32 {
        use crate::flags::errno as e;
        use Errno::*;
        match self {
            EPERM => e::EPERM,
            ENOENT => e::ENOENT,
            ESRCH => e::ESRCH,
            EINTR => e::EINTR,
            EIO => e::EIO,
            ENXIO => e::ENXIO,
            E2BIG => e::E2BIG,
            ENOEXEC => e::ENOEXEC,
            EBADF => e::EBADF,
            ECHILD => e::ECHILD,
            EAGAIN => e::EAGAIN,
            ENOMEM => e::ENOMEM,
            EACCES => e::EACCES,
            EFAULT => e::EFAULT,
            ENOTBLK => e::ENOTBLK,
            EBUSY => e::EBUSY,
            EEXIST => e::EEXIST,
            EXDEV => e::EXDEV,
            ENODEV => e::ENODEV,
            ENOTDIR => e::ENOTDIR,
            EISDIR => e::EISDIR,
            EINVAL => e::EINVAL,
            ENFILE => e::ENFILE,
            EMFILE => e::EMFILE,
            ENOTTY => e::ENOTTY,
            ETXTBSY => e::ETXTBSY,
            EFBIG => e::EFBIG,
            ENOSPC => e::ENOSPC,
            ESPIPE => e::ESPIPE,
            EROFS => e::EROFS,
            EMLINK => e::EMLINK,
            EPIPE => e::EPIPE,
            EDOM => e::EDOM,
            ERANGE => e::ERANGE,
            EPROTO => e::EPROTO,
            EOVERFLOW => e::EOVERFLOW,
            Unknown(x) => x,
        }
    }
}

/// 统一错误类型：保存 Errno；Display 里带上 errno 文案
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error {
    pub errno: Errno,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Errno::*;
        let name = match self.errno {
            EPERM => "EPERM",
            ENOENT => "ENOENT",
            ESRCH => "ESRCH",
            EINTR => "EINTR",
            EIO => "EIO",
            ENXIO => "ENXIO",
            E2BIG => "E2BIG",
            ENOEXEC => "ENOEXEC",
            EBADF => "EBADF",
            ECHILD => "ECHILD",
            EAGAIN => "EAGAIN",
            ENOMEM => "ENOMEM",
            EACCES => "EACCES",
            EFAULT => "EFAULT",
            ENOTBLK => "ENOTBLK",
            EBUSY => "EBUSY",
            EEXIST => "EEXIST",
            EXDEV => "EXDEV",
            ENODEV => "ENODEV",
            ENOTDIR => "ENOTDIR",
            EISDIR => "EISDIR",
            EINVAL => "EINVAL",
            ENFILE => "ENFILE",
            EMFILE => "EMFILE",
            ENOTTY => "ENOTTY",
            ETXTBSY => "ETXTBSY",
            EFBIG => "EFBIG",
            ENOSPC => "ENOSPC",
            ESPIPE => "ESPIPE",
            EROFS => "EROFS",
            EMLINK => "EMLINK",
            EPIPE => "EPIPE",
            EDOM => "EDOM",
            ERANGE => "ERANGE",
            EPROTO => "EPROTO",
            EOVERFLOW => "EOVERFLOW",
            Unknown(x) => return write!(f, "Unknown errno {}", x),
        };
        write!(f, "{}", name)
    }
}
impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        let raw = e.raw_os_error().unwrap_or(-1);
        Error {
            errno: Errno::from_raw(raw),
        }
    }
}

impl From<Error> for io::Error {
    fn from(e: Error) -> Self {
        // 尽力还原成一个带 raw_os_error 的 io::Error（Unknown 则用 Other）
        match e.errno {
            Errno::Unknown(_) => io::Error::new(io::ErrorKind::Other, e),
            _ => io::Error::from_raw_os_error(e.errno.to_raw()),
        }
    }
}

impl Error {
    #[inline]
    pub fn from_errno(errno: Errno) -> Self {
        Self { errno }
    }

    #[inline]
    pub fn invalid_data() -> Self {
        Self {
            errno: Errno::EPROTO,
        }
    }

    #[inline]
    pub fn truncated() -> Self {
        Self {
            errno: Errno::EOVERFLOW,
        }
    }
}

/// 统一 Result
pub(crate) type Result<T> = core::result::Result<T, Error>;

mod sealed {
    pub trait Sealed {}
    impl Sealed for i32 {}
    impl Sealed for isize {}
}

pub trait RetVal: sealed::Sealed + PartialOrd {
    const ZERO: Self;
}

// 在具体类型里把 ZERO 定义为 0
impl RetVal for i32 {
    const ZERO: Self = 0;
}
impl RetVal for isize {
    const ZERO: Self = 0;
}

#[inline]
pub(crate) fn retry_eintr<F, T>(mut f: F) -> Result<T>
where
    F: FnMut() -> T,
    T: RetVal,
{
    loop {
        let rc = f();
        if rc < T::ZERO {
            let err = io::Error::last_os_error();
            if err.raw_os_error() == Some(e::EINTR) {
                continue;
            }
            return Err(Error::from(err));
        } else {
            return Ok(rc);
        }
    }
}
