// src/watcher.rs

use anyhow::{Context, Result};
use crossbeam_channel::Sender;
use log::error;
use notify::RecursiveMode;
use notify_debouncer_mini::{
    new_debouncer, notify::RecommendedWatcher, DebounceEventResult, DebouncedEvent, Debouncer,
};
use std::{path::PathBuf, time::Duration};

/// 创建防抖 watcher，返回 Debouncer<RecommendedWatcher>
/// 并在后台线程消费底层原始事件以防阻塞
pub fn build_watcher(
    dir: PathBuf,
    tx: Sender<DebouncedEvent>,
    debounce_ms: u64,
) -> Result<Debouncer<RecommendedWatcher>> {
    let mut debouncer = new_debouncer(
        Duration::from_millis(debounce_ms),
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                for ev in events {
                    if let Err(e) = tx.send(ev) {
                        error!("发送去抖事件失败: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("监视器错误: {:?}", e);
            }
        },
    )
    .context(format!("创建监视器失败: {:?}", dir))?;

    // 递归监视目标目录
    debouncer
        .watcher()
        .watch(&dir, RecursiveMode::Recursive)
        .context(format!("监视目录 {:?} 失败", dir))?;

    Ok(debouncer)
}
