use common::Side;
use inotify::{Inotify, WatchDescriptor, WatchMask};
use std::{
    io,
    path::{Path, PathBuf},
};
use thiserror::Error;
use walkdir::WalkDir;

/// 轻量 inotify 上下文：后续你用 `inotify.read_events_blocking()` 或 `read_events()` 消费即可。
pub struct InotifyCtx {
    /// inotify 句柄
    pub inotify: Inotify,
    /// 归一化后的根目录
    pub root: PathBuf,
    /// A or B
    pub side: Side,
    /// 已添加的 (WatchDescriptor, 目录路径) 列表（用于事件 wd→路径反解/调试）
    pub watches: Vec<(WatchDescriptor, PathBuf)>,
    /// 实际使用的掩码（你的 mask 叠加了 ONLYDIR | EXCL_UNLINK）
    pub effective_mask: WatchMask,
    /// 简要统计
    pub stats: InotifyStats,
    /// 未能添加 watch 的目录（便于可观测性；对事件处理无额外负担）
    pub failed_dirs: Vec<(PathBuf, String)>,
}

pub struct InotifyStats {
    pub total_dirs: usize,   // 遍历到的目录总数（含 root）
    pub watched_dirs: usize, // 成功添加 watch 的目录数
}

#[derive(Debug, Error)]
pub enum InoInitError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("inotify init failed: {0}")]
    InotifyInit(String),
}

///
/// 初始化 inotify 并对 `root` 下所有**目录**递归添加 watch。
/// - `root`: 根目录路径
/// - `mask`: 你关注的事件掩码（本函数会额外叠加 `ONLYDIR | EXCL_UNLINK`）
///
/// 返回一个轻量 `InotifyCtx`，不包含对后续事件无意义的冗余信息。
///
pub fn init_inotify_recursive<P: AsRef<Path>>(
    root: P,
    side: Side,
    mask: WatchMask,
) -> Result<InotifyCtx, InoInitError> {
    // 1) 归一化根路径（绝对路径）
    let root = root.as_ref().canonicalize()?;

    // 2) 初始化 inotify（crate 里没有 InitFlags；是否阻塞由后续读法决定）
    let mut ino = Inotify::init().map_err(|e| InoInitError::InotifyInit(e.to_string()))?;

    // 3) 目录监听最佳实践：只监听目录、忽略已被 unlink 的别名
    let effective_mask = mask | WatchMask::ONLYDIR | WatchMask::EXCL_UNLINK;

    // 4) 递归遍历并添加目录 watch
    let mut watches: Vec<(WatchDescriptor, PathBuf)> = Vec::new();
    let mut failed: Vec<(PathBuf, String)> = Vec::new();
    let mut total_dirs: usize = 0;
    let mut watched_dirs: usize = 0;

    for ent in WalkDir::new(&root).follow_links(false).into_iter() {
        let ent = match ent {
            Ok(e) => e,
            Err(e) => {
                let p = e
                    .path()
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|| PathBuf::from("<unknown>"));
                failed.push((p, e.to_string()));
                continue;
            }
        };
        if !ent.file_type().is_dir() {
            continue;
        }
        total_dirs += 1;
        let dir = ent.path().to_path_buf();

        match {
            let this = &mut ino;
            let path = &dir;
            this.watches().add(path, effective_mask)
        } {
            Ok(wd) => {
                watches.push((wd, dir));
                watched_dirs += 1;
            }
            Err(e) => {
                failed.push((dir, e.to_string()));
            }
        }
    }

    Ok(InotifyCtx {
        inotify: ino,
        root,
        side,
        watches,
        effective_mask,
        stats: InotifyStats {
            total_dirs,
            watched_dirs,
        },
        failed_dirs: failed,
    })
}
