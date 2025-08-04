// src/logger.rs
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};
use std::{fs::File, path::Path};

/// 初始化日志：优先尝试终端输出，然后写入文件，所有错误均捕获并打印到 stderr
pub fn init_logging(path: &Path) {
    // 确保日志文件父目录存在
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("Unable to create log directory {:?}: {e}", parent.display());
            // 继续尝试创建文件
        }
    }

    // 打开日志文件
    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Unable to create log file {:?}: {e}", path.display());
            return;
        }
    };

    // 收集可用的 logger
    let mut loggers: Vec<Box<dyn simplelog::SharedLogger>> = Vec::new();

    let term_logger = TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );
    loggers.push(term_logger);

    // 始终加入文件 logger
    loggers.push(WriteLogger::new(
        LevelFilter::Debug,
        Config::default(),
        file,
    ));

    // 初始化全局 logger
    if let Err(e) = CombinedLogger::init(loggers) {
        eprintln!("Log initialization failed: {e}");
    }
}
