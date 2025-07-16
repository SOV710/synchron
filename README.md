````markdown
# syncing

## 项目简介
`syncing` 是一个用 Rust 编写的双向实时文件夹同步工具，支持防抖、排除规则和日志记录。它监视两个本地目录中的变化，并自动将发生变更的文件或目录同步到另一端。

## 功能特性
- **双向同步**：任意一侧的新增、修改或删除都会自动反向同步。
- **实时监控**：基于 `notify-debouncer-mini` 实现防抖监控，支持可配置的防抖时长。
- **排除模式**：根目录及其所有子目录中支持 `.syncingignore` 文件，用 Glob 模式忽略指定文件或文件夹。
- **日志记录**：内置 `simplelog`，同时输出到终端和配置文件（默认 `~/.local/share/syncing/syncing.log`），可自定义日志级别和路径。
- **命令行配置**：通过 `clap` 提供友好的 CLI 界面，支持参数校验与帮助文档。

## 安装步骤
```bash
# 克隆仓库
git clone https://github.com/yourusername/syncing.git
cd syncing

# 使用 Rust 工具链编译并安装
cargo install --path .
````

## 使用示例

```bash
# 基本用法：同步目录 A 和 目录 B
syncing /path/to/dirA /path/to/dirB

# 自定义防抖时长（毫秒）
syncing /dirA /dirB --debounce-ms 300

# 指定日志文件路径
syncing /dirA /dirB --log-file /home/user/.local/share/syncing/sync.log
```

## 配置说明

* `DIR_A`、`DIR_B`：要同步的两个目录，必须事先存在。
* `--debounce-ms <ms>`：防抖时长，单位毫秒，范围 1–60000，默认 500。
* `--log-file <path>`：日志文件路径，默认为 `$HOME/.local/share/syncing/syncing.log`。
* `.syncingignore`：放在任意同步目录及子目录中，内容与 `.gitignore` 语法兼容，用于忽略不需要同步的文件/文件夹。

## 贡献指南

1. Fork 本仓库并创建 feature 分支：`git checkout -b feature/xxx`
2. 编写代码并添加相应单元/集成测试：`cargo test`
3. 提交 PR，并填写清晰的变更描述与测试步骤
4. 保持代码风格一致，建议通过 `cargo fmt` 和 `cargo clippy`

## 许可证信息

本项目遵循 MIT 许可证，详见 [LICENSE](./LICENSE) 文件。

```
::contentReference[oaicite:0]{index=0}
```
