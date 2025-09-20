#![allow(non_camel_case_types, non_upper_case_globals)]
use core::mem;
#[repr(C)]
pub struct fanotify_event_metadata {
    pub event_len: u32,
    pub vers: u8,
    pub reserved: u8,
    pub metadata_len: u16,
    pub mask: u64, // __aligned_u64
    pub fd: i32,
    pub pid: i32,
}

// ===========================
// Event mask (u64) – user-space can register for
// ===========================
pub const FAN_ACCESS: u64 = 0x0000_0001;
pub const FAN_MODIFY: u64 = 0x0000_0002;
pub const FAN_ATTRIB: u64 = 0x0000_0004;
pub const FAN_CLOSE_WRITE: u64 = 0x0000_0008;
pub const FAN_CLOSE_NOWRITE: u64 = 0x0000_0010;
pub const FAN_OPEN: u64 = 0x0000_0020;
pub const FAN_MOVED_FROM: u64 = 0x0000_0040;
pub const FAN_MOVED_TO: u64 = 0x0000_0080;
pub const FAN_CREATE: u64 = 0x0000_0100;
pub const FAN_DELETE: u64 = 0x0000_0200;
pub const FAN_DELETE_SELF: u64 = 0x0000_0400;
pub const FAN_MOVE_SELF: u64 = 0x0000_0800;
pub const FAN_OPEN_EXEC: u64 = 0x0000_1000;

pub const FAN_Q_OVERFLOW: u64 = 0x0000_4000;
// pub const FAN_FS_ERROR: u64 = 0x0000_8000;

pub const FAN_OPEN_PERM: u64 = 0x0001_0000;
pub const FAN_ACCESS_PERM: u64 = 0x0002_0000;
pub const FAN_OPEN_EXEC_PERM: u64 = 0x0004_0000;

pub const FAN_EVENT_ON_CHILD: u64 = 0x0800_0000;
pub const FAN_RENAME: u64 = 0x1000_0000;
pub const FAN_ONDIR: u64 = 0x4000_0000;

// helper events
pub const FAN_CLOSE: u64 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE;
// pub const FAN_MOVE: u64 = FAN_MOVED_FROM | FAN_MOVED_TO;

// Deprecated convenience sets (保留与 C 一致，勿在新代码使用)
// pub const FAN_ALL_EVENTS: u64 = FAN_ACCESS | FAN_MODIFY | FAN_CLOSE | FAN_OPEN;
// pub const FAN_ALL_PERM_EVENTS: u64 = FAN_OPEN_PERM | FAN_ACCESS_PERM;
// pub const FAN_ALL_OUTGOING_EVENTS: u64 = FAN_ALL_EVENTS | FAN_ALL_PERM_EVENTS | FAN_Q_OVERFLOW;

// ===========================
// fanotify_init() flags (u32) – 不是 mask
// ===========================
pub const FAN_CLOEXEC: u32 = 0x0000_0001;
pub const FAN_NONBLOCK: u32 = 0x0000_0002;

// class（两位互斥）：
pub const FAN_CLASS_NOTIF: u32 = 0x0000_0000;
pub const FAN_CLASS_CONTENT: u32 = 0x0000_0004;
pub const FAN_CLASS_PRE_CONTENT: u32 = 0x0000_0008;

// resource limits / audit
pub const FAN_UNLIMITED_QUEUE: u32 = 0x0000_0010;
pub const FAN_UNLIMITED_MARKS: u32 = 0x0000_0020;
// pub const FAN_ENABLE_AUDIT: u32 = 0x0000_0040;

// event record format controls
pub const FAN_REPORT_PIDFD: u32 = 0x0000_0080;
pub const FAN_REPORT_TID: u32 = 0x0000_0100;
pub const FAN_REPORT_FID: u32 = 0x0000_0200;
pub const FAN_REPORT_DIR_FID: u32 = 0x0000_0400;
pub const FAN_REPORT_NAME: u32 = 0x0000_0800;
pub const FAN_REPORT_TARGET_FID: u32 = 0x0000_1000;
pub const FAN_REPORT_FD_ERROR: u32 = 0x0000_2000;

// convenience macros
pub const FAN_REPORT_DFID_NAME: u32 = FAN_REPORT_DIR_FID | FAN_REPORT_NAME;
pub const FAN_REPORT_DFID_NAME_TARGET: u32 =
    FAN_REPORT_DFID_NAME | FAN_REPORT_FID | FAN_REPORT_TARGET_FID;

// Deprecated aggregate
// pub const FAN_ALL_CLASS_BITS: u32 = FAN_CLASS_NOTIF | FAN_CLASS_CONTENT | FAN_CLASS_PRE_CONTENT;
// pub const FAN_ALL_INIT_FLAGS: u32 = FAN_CLOEXEC | FAN_NONBLOCK | FAN_ALL_CLASS_BITS | FAN_UNLIMITED_QUEUE | FAN_UNLIMITED_MARKS;

// ===========================
// fanotify_modify_mark() flags (u32)
// ===========================
pub const FAN_MARK_ADD: u32 = 0x0000_0001;
pub const FAN_MARK_REMOVE: u32 = 0x0000_0002;
pub const FAN_MARK_DONT_FOLLOW: u32 = 0x0000_0004;
pub const FAN_MARK_ONLYDIR: u32 = 0x0000_0008;
// 0x0000_0010 reserved for FAN_MARK_MOUNT (see below)
pub const FAN_MARK_IGNORED_MASK: u32 = 0x0000_0020;
pub const FAN_MARK_IGNORED_SURV_MODIFY: u32 = 0x0000_0040;
pub const FAN_MARK_FLUSH: u32 = 0x0000_0080;
// 0x0000_0100 reserved for FAN_MARK_FILESYSTEM (see below)
pub const FAN_MARK_EVICTABLE: u32 = 0x0000_0200;
// mutually exclusive with FAN_MARK_IGNORED_MASK
pub const FAN_MARK_IGNORE: u32 = 0x0000_0400;

// mark type（不是按位标志，二者/三者互斥，依 C 约定）
pub const FAN_MARK_INODE: u32 = 0x0000_0000;
pub const FAN_MARK_MOUNT: u32 = 0x0000_0010;
pub const FAN_MARK_FILESYSTEM: u32 = 0x0000_0100;

// convenience
pub const FAN_MARK_IGNORE_SURV: u32 = FAN_MARK_IGNORE | FAN_MARK_IGNORED_SURV_MODIFY;

// Deprecated aggregate
pub const FAN_ALL_MARK_FLAGS: u32 = FAN_MARK_ADD
    | FAN_MARK_REMOVE
    | FAN_MARK_DONT_FOLLOW
    | FAN_MARK_ONLYDIR
    | FAN_MARK_MOUNT
    | FAN_MARK_IGNORED_MASK
    | FAN_MARK_IGNORED_SURV_MODIFY
    | FAN_MARK_FLUSH;

// ===========================
// Responses to a _PERM event
// ===========================

pub const FAN_ALLOW: u32 = 0x01;
pub const FAN_DENY: u32 = 0x02;
pub const FAN_AUDIT: u32 = 0x10; // bitmask to create audit record for result
pub const FAN_INFO: u32 = 0x20; // bitmask to indicate additional information

// No fd set in event
pub const FAN_NOFD: i32 = -1;
pub const FAN_NOPIDFD: i32 = FAN_NOFD;
pub const FAN_EPIDFD: i32 = -2;

// ===========================
// Misc numbers
// ===========================
pub const FANOTIFY_METADATA_VERSION: u8 = 3;

// event info types (u8 in practice; keep as u8/u32 均可用作常量)
pub const FAN_EVENT_INFO_TYPE_FID: u8 = 1;
pub const FAN_EVENT_INFO_TYPE_DFID_NAME: u8 = 2;
pub const FAN_EVENT_INFO_TYPE_DFID: u8 = 3;
pub const FAN_EVENT_INFO_TYPE_PIDFD: u8 = 4;
pub const FAN_EVENT_INFO_TYPE_ERROR: u8 = 5;
// Special for FAN_RENAME
pub const FAN_EVENT_INFO_TYPE_OLD_DFID_NAME: u8 = 10;
// 11 reserved (OLD_DFID)
pub const FAN_EVENT_INFO_TYPE_NEW_DFID_NAME: u8 = 12;
// 13 reserved (NEW_DFID)

// ========== sanity checks ==========
const _: () = {
    // C layout sanity
    assert!(mem::size_of::<fanotify_event_metadata>() == 24);
    assert!(mem::align_of::<fanotify_event_metadata>() >= 8);
};
