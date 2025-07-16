# synchron

synchron 是一个同步本地多个文件夹的工具，支持多向实时传输，启发自 syncthing

## Installation

### Install Binaries

| Distribution            | Instructions                                                           |
| ----------------------- | ---------------------------------------------------------------------- |
| **GitHub Release**      | 从 [Releases](https://github.com/SOV710/synchron/releases/) 下载对应平台包并解压 |
| `.exe` (Windows)        | 下载 `synchron-windows-x86_64.exe`                                       |
| `.dmg` (macOS)          | 下载 `synchron-macos.dmg` 并挂载安装                                          |
| `.deb` (Debian/Ubuntu)` | `sudo dpkg -i synchron_*.deb`                                          |
| `.rpm` (Fedora/CentOS)  | `sudo rpm -i synchron-*.rpm`                                           |
| `.pkg.tar.xz` (Arch)    | `sudo pacman -U synchron-*.pkg.tar.xz`                                 |
| `.tar.gz` (Linux)       | `tar -xzf synchron-*.tar.gz && mv synchron /usr/local/bin/`            |

### Cargo

```bash
cargo install synchron
```

## Configuration

### Flags

- `--debounce-ms <ms>`：防抖时长（毫秒），范围 1–60000，默认 500

- `--log-file <path>`：日志文件路径，默认 `$HOME/.local/share/syncing/syncing.log`

## LICENSE

MIT
