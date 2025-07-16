// tests/integration.rs
use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli_sync_roundtrip() {
    let dir_a = assert_fs::TempDir::new().unwrap();
    let dir_b = assert_fs::TempDir::new().unwrap();

    // 在 dir_a 中创建文件
    dir_a.child("x.txt").write_str("ABC").unwrap();

    // 运行 syncing 命令
    let mut cmd = Command::cargo_bin("syncing").unwrap();
    cmd.arg(dir_a.path())
       .arg(dir_b.path())
       // 使用短防抖以加快测试
       .arg("--debounce-ms").arg("10")
       .assert().success();

    // 给后台 watcher 一点时间
    std::thread::sleep(std::time::Duration::from_millis(50));

    // 验证文件同步到 dir_b
    dir_b.child("x.txt").assert(predicate::path::exists());
    dir_b.child("x.txt").assert(predicate::str::contains("ABC"));
}
