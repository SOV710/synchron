use crate::error::*;
use crate::flags::epoll as eflag;
use crate::raw;
use crate::types::*;
use std::mem::MaybeUninit;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, FromRawFd, OwnedFd};
use std::time::Duration;

/// ---------- Strong-typed flag newtypes ----------

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpollCreateFlags(pub i32);
impl EpollCreateFlags {
    pub const EMPTY: Self = Self(0);
    pub const CLOEXEC: Self = Self(eflag::EPOLL_CLOEXEC);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpollCtlOp(pub i32);
impl EpollCtlOp {
    pub const ADD: Self = Self(eflag::EPOLL_CTL_ADD);
    pub const MOD: Self = Self(eflag::EPOLL_CTL_MOD);
    pub const DEL: Self = Self(eflag::EPOLL_CTL_DEL);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct EpollEventFlags(pub u32);
impl EpollEventFlags {
    pub const EMPTY: Self = Self(0);
    pub const IN: Self = Self(eflag::EPOLLIN);
    pub const OUT: Self = Self(eflag::EPOLLOUT);
    pub const ERR: Self = Self(eflag::EPOLLERR);
    pub const HUP: Self = Self(eflag::EPOLLHUP);
    pub const RDHUP: Self = Self(eflag::EPOLLRDHUP);
    pub const PRI: Self = Self(eflag::EPOLLPRI);
    pub const ET: Self = Self(eflag::EPOLLET);
    pub const ONESHOT: Self = Self(eflag::EPOLLONESHOT);
}
impl core::ops::BitOr for EpollEventFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl core::ops::BitOrAssign for EpollEventFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Strong-typed ready event returned by epoll.
#[derive(Copy, Clone, Debug)]
pub struct ReadyEvent {
    pub flags: EpollEventFlags,
    /// user data echoed back by kernel; you decide语义（fd、指针、标识等）
    pub data_u64: u64,
}

/// RAII epoll handle.
pub struct Epoll {
    fd: OwnedFd,
}

impl Epoll {
    /// Create epoll instance (RAII).
    pub fn new(flags: EpollCreateFlags) -> Result<Self> {
        let fd: i32 = retry_eintr(|| unsafe {
            {
                raw::epoll_create1(flags.0 as c_int)
            }
        })?;
        let owned = unsafe { OwnedFd::from_raw_fd(fd as RawFd) };
        Ok(Self { fd: owned })
    }

    /// Register a target fd with event mask and user data (u64).
    pub fn add(&self, target: impl AsFd, events: EpollEventFlags, data_u64: u64) -> Result<()> {
        self.ctl_inner(EpollCtlOp::ADD, target, events, data_u64)
    }

    /// Modify an existing registration.
    pub fn modify(&self, target: impl AsFd, events: EpollEventFlags, data_u64: u64) -> Result<()> {
        self.ctl_inner(EpollCtlOp::MOD, target, events, data_u64)
    }

    /// Delete a registration.
    pub fn delete(&self, target: impl AsFd) -> Result<()> {
        // data is ignored for DEL; pass a dummy event ptr per API
        let mut ev = EpollEvent {
            events: 0,
            data: EpollData { u64_: 0 },
        };
        retry_eintr(|| unsafe {
            raw::epoll_ctl(
                self.fd.as_raw_fd() as c_int,
                EpollCtlOp::DEL.0 as c_int,
                target.as_fd().as_raw_fd() as c_int,
                &mut ev as *mut _,
            )
        })?;

        Ok(())
    }

    /// Wait with millisecond timeout. `timeout_ms = -1` means infinite.
    pub fn wait(&self, max_events: usize, timeout_ms: i32) -> Result<Vec<ReadyEvent>> {
        let mut buf: Vec<MaybeUninit<EpollEvent>> = Vec::with_capacity(max_events);
        // Safety: set_len to let kernel write into the slice.
        unsafe {
            buf.set_len(max_events);
        }

        let n = retry_eintr(|| unsafe {
            raw::epoll_wait(
                self.fd.as_raw_fd() as c_int,
                buf.as_mut_ptr() as *mut EpollEvent,
                max_events as c_int,
                timeout_ms as c_int,
            )
        })?;

        Ok(unsafe { transmute_events(&buf[..n as usize]) })
    }

    /// Wait with temporary signal mask.
    pub fn pwait(
        &self,
        max_events: usize,
        timeout_ms: i32,
        mask: Option<&SigSet>,
    ) -> Result<Vec<ReadyEvent>> {
        let mut buf: Vec<MaybeUninit<EpollEvent>> = Vec::with_capacity(max_events);
        unsafe {
            buf.set_len(max_events);
        }

        let n = retry_eintr(|| unsafe {
            raw::epoll_pwait(
                self.fd.as_raw_fd() as c_int,
                buf.as_mut_ptr() as *mut EpollEvent,
                max_events as c_int,
                timeout_ms as c_int,
                mask.map(|m| m as *const _).unwrap_or(core::ptr::null()),
            )
        })?;

        Ok(unsafe { transmute_events(&buf[..n as usize]) })
    }

    /// Wait with relative timeout as `Duration` (uses epoll_pwait2 if available).
    pub fn pwait2(
        &self,
        max_events: usize,
        timeout: Option<Duration>,
        mask: Option<&SigSet>,
    ) -> Result<Vec<ReadyEvent>> {
        let mut buf: Vec<MaybeUninit<EpollEvent>> = Vec::with_capacity(max_events);
        unsafe {
            buf.set_len(max_events);
        }

        let ts = timeout.map(|d| TimeSpec {
            tv_sec: d.as_secs() as i64,
            tv_nsec: d.subsec_nanos() as i64,
        });

        let n = retry_eintr(|| unsafe {
            raw::epoll_pwait2(
                self.fd.as_raw_fd() as c_int,
                buf.as_mut_ptr() as *mut EpollEvent,
                max_events as c_int,
                ts.as_ref()
                    .map(|t| t as *const _)
                    .unwrap_or(core::ptr::null()),
                mask.map(|m| m as *const _).unwrap_or(core::ptr::null()),
            )
        })?;

        Ok(unsafe { transmute_events(&buf[..n as usize]) })
    }

    /// Internal helper for ADD/MOD.
    fn ctl_inner(
        &self,
        op: EpollCtlOp,
        target: impl AsFd,
        events: EpollEventFlags,
        data_u64: u64,
    ) -> Result<()> {
        let mut ev = EpollEvent {
            events: events.0,
            data: EpollData { u64_: data_u64 },
        };
        retry_eintr(|| unsafe {
            raw::epoll_ctl(
                self.fd.as_raw_fd() as c_int,
                op.0 as c_int,
                target.as_fd().as_raw_fd() as c_int,
                &mut ev as *mut _,
            )
        })?;
        Ok(())
    }

    /// Borrow as `BorrowedFd` for advanced interop (rarely needed).
    pub fn as_borrowed(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

impl AsFd for Epoll {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.fd.as_fd()
    }
}

/// SAFETY: Convert the kernel-filled epoll_event array into safe `ReadyEvent`s.
unsafe fn transmute_events(raws: &[MaybeUninit<EpollEvent>]) -> Vec<ReadyEvent> {
    let mut out = Vec::with_capacity(raws.len());
    for e in raws {
        let ev = e.assume_init_ref();
        // SAFETY: reading union as u64 (what we wrote) is ok.
        let data = unsafe { ev.data.u64_ };
        out.push(ReadyEvent {
            flags: EpollEventFlags(ev.events),
            data_u64: data,
        });
    }
    out
}
