// src/sync.rs
use crate::cli::Opt;
use anyhow::{Context, Result};
use log::{debug, trace};
use std::{fs, path::Path};

pub fn handle_event(path: &Path, opt: &Opt) -> Result<()> {
    // 1. 仅处理 A 或 B 目录下的事件
    let (src_root, dst_root) = if path.starts_with(&opt.dir_a) {
        (&opt.dir_a, &opt.dir_b)
    } else if path.starts_with(&opt.dir_b) {
        (&opt.dir_b, &opt.dir_a)
    } else {
        trace!("忽略非监控路径事件: {:?}", path);
        return Ok(());
    };

    // 2. 计算相对路径并跳过根目录自身事件
    let rel = path
        .strip_prefix(src_root)
        .with_context(|| format!("strip_prefix 失败: {:?}", path))?;
    if rel.as_os_str().is_empty() {
        trace!("忽略根目录事件: {:?}", path);
        return Ok(());
    }

    // 3. 目标路径
    let dst_path = dst_root.join(rel);

    // 4. 源存在：处理创建／复制（并过滤符号链接）
    if path.exists() {
        // 4.1 仅在存在的情况下，读取元数据判断是否为符号链接
        let meta = fs::symlink_metadata(path)
            .with_context(|| format!("读取文件元数据失败: {:?}", path))?;
        if meta.file_type().is_symlink() {
            debug!("忽略符号链接: {:?}", path);
            return Ok(());
        }

        // 4.2 目录 vs 文件
        if meta.file_type().is_dir() {
            fs::create_dir_all(&dst_path)
                .with_context(|| format!("创建目录失败: {:?}", dst_path))?;
            debug!("创建目录: {:?}", dst_path);
        } else {
            if let Some(parent) = dst_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("创建父目录失败: {:?}", parent))?;
            }
            fs::copy(path, &dst_path)
                .with_context(|| format!("复制文件失败 {:?} -> {:?}", path, dst_path))?;
            debug!("复制文件: {:?} -> {:?}", path, dst_path);
        }
    }
    // 5. 源不存在且目标存在：删除目标
    else if dst_path.exists() {
        if dst_path.is_dir() {
            fs::remove_dir_all(&dst_path)
                .with_context(|| format!("删除目录失败: {:?}", dst_path))?;
            debug!("删除目录: {:?}", dst_path);
        } else {
            fs::remove_file(&dst_path).with_context(|| format!("删除文件失败: {:?}", dst_path))?;
            debug!("删除文件: {:?}", dst_path);
        }
    }

    Ok(())
}

// unit testing
#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::Opt;
    use std::fs;
    use tempfile::TempDir;

    fn make_opt(a: &TempDir, b: &TempDir) -> Opt {
        Opt {
            dir_a: a.path().to_path_buf(),
            dir_b: b.path().to_path_buf(),
            debounce_ms: 500,
            log_file: std::env::temp_dir().join("sync.log"),
        }
    }

    #[test]
    fn test_handle_event_copy() {
        let ta = TempDir::new().unwrap();
        let tb = TempDir::new().unwrap();
        let opt = make_opt(&ta, &tb);
        let src = ta.path().join("foo.txt");
        fs::write(&src, "hello").unwrap();
        // 手动调用
        handle_event(&src, &opt).unwrap();
        let dst = tb.path().join("foo.txt");
        assert!(dst.exists());
        assert_eq!(fs::read_to_string(dst).unwrap(), "hello");
    }

    #[test]
    fn test_handle_event_delete() {
        let ta = TempDir::new().unwrap();
        let tb = TempDir::new().unwrap();
        let opt = make_opt(&ta, &tb);
        let src = ta.path().join("foo.txt");
        let dst = tb.path().join("foo.txt");
        fs::write(&src, "x").unwrap();
        handle_event(&src, &opt).unwrap();
        assert!(dst.exists());
        // 删除 src，再同步
        fs::remove_file(&src).unwrap();
        handle_event(&src, &opt).unwrap();
        assert!(!dst.exists());
    }
}
