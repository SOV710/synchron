// src/main.rs

mod cli;
mod ignore_config;
mod logger;
mod sync;
mod watcher;

use clap::Parser;
use crossbeam_channel::unbounded;
use ignore_config::IgnoreMatcher;
use log::{debug, error};
use logger::init_logging;
use notify_debouncer_mini::DebouncedEvent;
use std::process;
use sync::handle_event;
use watcher::build_watcher;

fn main() {
    // 1. 解析命令行参数
    let opt = cli::Opt::parse();

    // 2. 初始化日志
    init_logging(&opt.log_file);

    // 3. 加载各目录的 .synchronignore 规则
    let matcher_a = IgnoreMatcher::from_dir(&opt.dir_a);
    let matcher_b = IgnoreMatcher::from_dir(&opt.dir_b);

    // 4. 创建事件通道
    let (tx, rx) = unbounded::<DebouncedEvent>();

    // 5. 创建并启动两个 watcher
    if let Err(e) = build_watcher(&opt.dir_a, tx.clone(), opt.debounce_ms) {
        error!("创建监视器 A 失败: {e}");
        process::exit(1);
    }
    if let Err(e) = build_watcher(&opt.dir_b, tx, opt.debounce_ms) {
        error!("创建监视器 B 失败: {e}");
        process::exit(1);
    }

    // 6. 事件循环：处理每一个去抖后事件
    for event in &rx {
        let path = &event.path;

        // 6.1 路径被忽略则跳过
        if matcher_a.is_ignored(path) || matcher_b.is_ignored(path) {
            debug!("忽略路径: {:?}", path.display());
            continue;
        }

        // 6.2 同步该文件或目录
        if let Err(e) = handle_event(path, &opt) {
            error!("同步失败 {:?}: {e}", path.display());
        }
    }
}
