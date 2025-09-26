#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#[cfg(target_os = "linux")]

pub const O_ACCMODE: i32 = 0o0000003;
pub const O_RDONLY: i32 = 0o0000000;
pub const O_WRONLY: i32 = 0o0000001;
pub const O_RDWR: i32 = 0o0000002;

// “Not fcntl.” 仅 open 用
pub const O_CREAT: i32 = 0o0000100;
pub const O_EXCL: i32 = 0o0000200;
pub const O_NOCTTY: i32 = 0o0000400;
pub const O_TRUNC: i32 = 0o0001000;

pub const O_APPEND: i32 = 0o0002000;
pub const O_NONBLOCK: i32 = 0o0004000;
pub const O_NDELAY: i32 = O_NONBLOCK;

// 同步/异步
pub const __O_DSYNC: i32 = 0o00010000; // 0x0010_00
pub const O_DSYNC: i32 = __O_DSYNC;
pub const O_ASYNC: i32 = 0o00020000; // 0x0020_00
pub const __O_DIRECT: i32 = 0o00040000; // 0x0040_00
pub const O_DIRECT: i32 = __O_DIRECT;

// 大文件支持（历史/架构相关）
pub const __O_LARGEFILE: i32 = 0o00100000; // 0x0080_00
pub const O_LARGEFILE: i32 = __O_LARGEFILE;

// 目录/跟随/… 内核内部位
pub const __O_DIRECTORY: i32 = 0o00200000; // 0x0100_00
pub const O_DIRECTORY: i32 = __O_DIRECTORY;
pub const __O_NOFOLLOW: i32 = 0o00400000; // 0x0200_00
pub const O_NOFOLLOW: i32 = __O_NOFOLLOW;

// 同步写
pub const O_SYNC: i32 = 0o004010000; // 0x0010_1000
pub const O_FSYNC: i32 = O_SYNC;

// 其它扩展位
pub const __O_DSYNC_dup: i32 = __O_DSYNC; // 仅为可读性，可删
pub const __O_NOATIME: i32 = 0o01000000; // 0x0400_00
pub const O_NOATIME: i32 = __O_NOATIME; // 0x0400_00
pub const __O_CLOEXEC: i32 = 0o02000000; // 0x0800_00
pub const O_CLOEXEC: i32 = __O_CLOEXEC; // 0x0800_00
pub const __O_PATH: i32 = 0o010000000; // 0x2000_00
pub const O_PATH: i32 = __O_PATH; // 0x2000_00

// tmpfile（与 __O_DIRECTORY 组合）
pub const __O_TMPFILE: i32 = 0o020000000 | __O_DIRECTORY; // 0x410000
pub const O_TMPFILE: i32 = __O_TMPFILE; // 0x410000

/// AT* field
/// Special value used to indicate "use current working directory"
pub const AT_FDCWD: i32 = -100;
/// Do not follow symbolic links
pub const AT_SYMLINK_NOFOLLOW: i32 = 0x100;
/// Remove directory instead of unlinking file
pub const AT_REMOVEDIR: i32 = 0x200;
/// Follow symbolic links
pub const AT_SYMLINK_FOLLOW: i32 = 0x400;
/// Test access using effective IDs, not real IDs (for faccessat)
pub const AT_EACCESS: i32 = 0x200;
// GNU extensions
/// Suppress terminal automount traversal
pub const AT_NO_AUTOMOUNT: i32 = 0x800;
/// Allow empty relative pathname
pub const AT_EMPTY_PATH: i32 = 0x1000;
/// Mask for statx() sync type flags
pub const AT_STATX_SYNC_TYPE: i32 = 0x6000;
/// Use stat()'s default sync type
pub const AT_STATX_SYNC_AS_STAT: i32 = 0x0000;
/// Force sync before statx()
pub const AT_STATX_FORCE_SYNC: i32 = 0x2000;
/// Do not sync before statx()
pub const AT_STATX_DONT_SYNC: i32 = 0x4000;
/// Apply operation to the entire subtree (e.g. renameat2)
pub const AT_RECURSIVE: i32 = 0x8000;
