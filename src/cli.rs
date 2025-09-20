// src/cli.rs
use clap::Parser;
use std::{env, ffi::OsString, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Opt {
    /// 同步目录 A
    #[arg(value_name = "DIR_A", value_parser = parse_dir)]
    pub dir_a: PathBuf,

    /// 同步目录 B
    #[arg(value_name = "DIR_B", value_parser = parse_dir)]
    pub dir_b: PathBuf,

    /// 防抖时长（毫秒），范围 1–60000
    #[arg(long, default_value_t = 500, value_parser = parse_debounce)]
    pub debounce_ms: u64,

    /// 日志文件路径
    #[arg(long, default_value_os_t = default_log_file())]
    pub log_file: PathBuf,
}

/// 自定义目录解析器：接收 &str，检查存在且为目录
fn parse_dir(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    if !p.exists() {
        return Err(format!("Path does not exist: {:?}", p));
    }
    if !p.is_dir() {
        return Err(format!("Path is not a directory: {:?}", p));
    }
    Ok(p)
}

/// 自定义防抖参数解析：检查整数且在 [1,60000] 范围内
fn parse_debounce(s: &str) -> Result<u64, String> {
    let v: u64 = s
        .parse()
        .map_err(|_| format!("debounce_ms must be an integer: {}", s))?;
    if v == 0 || v > 60_000 {
        Err("debounce_ms must be between 1 and 60000.".into())
    } else {
        Ok(v)
    }
}

/// 运行时计算默认日志路径：$HOME/.local/share/synchron/synchron.log
fn default_log_file() -> PathBuf {
    let home = env::var_os("HOME").unwrap_or_else(|| OsString::from("."));
    let mut p = PathBuf::from(home);
    p.push(".local/share/synchron/synchron.log");
    p
}

// unit testing
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dir_ok() {
        let dir = std::env::current_dir().unwrap();
        let s = dir.to_str().unwrap();
        assert_eq!(parse_dir(s).unwrap(), dir);
    }

    #[test]
    fn test_parse_dir_not_exist() {
        let err = parse_dir("/path/does/not/exist").unwrap_err();
        assert!(err.contains("Path does not exist"));
    }

    #[test]
    fn test_parse_debounce_ok() {
        assert_eq!(parse_debounce("123").unwrap(), 123);
    }
    #[test]
    fn test_parse_debounce_too_large() {
        assert!(parse_debounce("999999").is_err());
    }
}
