// src/ignore_config.rs
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use log::warn;
use std::path::{Path, PathBuf};

pub struct IgnoreMatcher {
    matcher: Gitignore,
    root: PathBuf,
}

impl IgnoreMatcher {
    pub fn from_dir(dir: &PathBuf) -> Self {
        // 构造 builder
        let mut builder = GitignoreBuilder::new(dir);

        // 收集从根到目标目录的所有 .synchronignore
        let mut ancestors = Vec::new();
        let mut cur = Some(dir.as_path());
        while let Some(p) = cur {
            ancestors.push(p.to_path_buf());
            cur = p.parent();
        }
        // 现在 ancestors = [dir, parent, ..., root]，反转为 root→dir
        for path in ancestors.into_iter().rev() {
            let ignore_file = path.join(".synchronignore");
            if ignore_file.exists() {
                if let Some(e) = builder.add(ignore_file.clone()) {
                    warn!("加载 .synchronignore {:?} 失败: {}", ignore_file, e);
                }
            }
        }

        // 构建 matcher，失败时警告并使用空规则
        let matcher = match builder.build() {
            Ok(m) => m,
            Err(e) => {
                warn!("构建 IgnoreMatcher 失败，忽略所有规则: {}", e);
                GitignoreBuilder::new(dir).build().unwrap()
            }
        };

        Self {
            matcher,
            root: dir.clone(),
        }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        // 仅对 root 内的路径做匹配
        match path.strip_prefix(&self.root) {
            Ok(rel) => self.matcher.matched(rel, path.is_dir()).is_ignore(),
            Err(_) => false,
        }
    }
}

// unit testing
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;

    #[test]
    fn test_ignorematcher_simple() {
        let d = tempdir().unwrap();
        let root = d.path().to_path_buf();
        // 写 .synchronignore 忽略 *.log
        fs::write(root.join(".synchronignore"), "*.log\n").unwrap();
        let im = IgnoreMatcher::from_dir(&root);
        let file1 = root.join("a.txt");
        let file2 = root.join("b.log");
        File::create(&file1).unwrap();
        File::create(&file2).unwrap();
        assert!(!im.is_ignored(&file1));
        assert!(im.is_ignored(&file2));
    }
}
