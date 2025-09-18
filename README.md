# synchron

<br/>
  :warning: <i>This project is in very early development state</i>
<br />

synchron is a tool for synchronizing multiple local folders, supporting multi-directional real-time transfers. Inspired by syncthing

## Installation

### Install Binaries

| Distribution           | Instructions                                                                                               |
| ---------------------- | ---------------------------------------------------------------------------------------------------------- |
| **GitHub Release**     | Download the appropriate package from [Releases](https://github.com/SOV710/synchron/releases/) and unpack |
| `.exe` (Windows)       | Download `synchron-windows-x86_64.exe`                                                                     |
| `.dmg` (macOS)         | Download `synchron-macos.dmg` and mount to install                                                         |
| `.deb` (Debian/Ubuntu) | Run `sudo dpkg -i synchron_*.deb`                                                                          |
| `.rpm` (Fedora/CentOS) | Run `sudo rpm -i synchron-*.rpm`                                                                           |
| `.pkg.tar.xz` (Arch)   | Run `sudo pacman -U synchron-*.pkg.tar.xz`                                                                 |
| `.tar.gz` (Linux)      | Extract with `tar -xzf synchron-*.tar.gz` and move `synchron` to `/usr/local/bin/`                         |

### Cargo

Use `cargo install synchron`

## Configuration

### Flags

* `--debounce-ms <ms>`: debounce duration in milliseconds (1–60000), default is 500
* `--log-file <path>`: log file path, default is `$HOME/.local/share/syncing/syncing.log`

## LICENSE

MIT
