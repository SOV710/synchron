// inotify_init1(IN_NONBLOCK | IN_CLOEXEC)
pub const IN_CLOEXEC: i32 = 0o2000000;
pub const IN_NONBLOCK: i32 = 0o4000;

// Supported events suitable for MASK parameter of inotify_add_watch.
pub const IN_ACCESS: u32 = 0x0000_0001; // File was accessed
pub const IN_MODIFY: u32 = 0x0000_0002; // File was modified
pub const IN_ATTRIB: u32 = 0x0000_0004; // Metadata changed
pub const IN_CLOSE_WRITE: u32 = 0x0000_0008; // Writable file was closed
pub const IN_CLOSE_NOWRITE: u32 = 0x0000_0010; // Unwritable file closed
pub const IN_CLOSE: u32 = IN_CLOSE_WRITE | IN_CLOSE_NOWRITE; // Close
pub const IN_OPEN: u32 = 0x0000_0020; // File was opened
pub const IN_MOVED_FROM: u32 = 0x0000_0040; // File was moved from X
pub const IN_MOVED_TO: u32 = 0x0000_0080; // File was moved to Y
pub const IN_MOVE: u32 = IN_MOVED_FROM | IN_MOVED_TO; // Moves
pub const IN_CREATE: u32 = 0x0000_0100; // Subfile was created
pub const IN_DELETE: u32 = 0x0000_0200; // Subfile was deleted
pub const IN_DELETE_SELF: u32 = 0x0000_0400; // Self was deleted
pub const IN_MOVE_SELF: u32 = 0x0000_0800; // Self was moved

// Events sent by the kernel.
pub const IN_UNMOUNT: u32 = 0x0000_2000; // Backing fs was unmounted
pub const IN_Q_OVERFLOW: u32 = 0x0000_4000; // Event queue overflowed
pub const IN_IGNORED: u32 = 0x0000_8000; // File was ignored

// Special flags.
pub const IN_ONLYDIR: u32 = 0x0100_0000; // Only watch if path is a dir
pub const IN_DONT_FOLLOW: u32 = 0x0200_0000; // Do not follow symlink
pub const IN_EXCL_UNLINK: u32 = 0x0400_0000; // Exclude events on unlinked obj
pub const IN_MASK_CREATE: u32 = 0x1000_0000; // Only create watches
pub const IN_MASK_ADD: u32 = 0x2000_0000; // Add to existing mask
pub const IN_ISDIR: u32 = 0x4000_0000; // Event occurred against dir
pub const IN_ONESHOT: u32 = 0x8000_0000; // Only send event once
