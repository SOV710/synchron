mod init_detect;
mod init_service;
mod uds;

use crate::init_detect::{detect, InitSystem};
use crate::init_service::*;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;
use time;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

// ======== Clap Args ========
#[derive(Parser, Debug)]
#[command(
    name = "synchron",
    version,
    about = "a lightweight Rust CLI tool for real‑time, bidirectional synchronization between two local directories."
)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Manage the synchron service (systemd/openrc)
    Service {
        #[command(subcommand)]
        service: Service,
    },

    /// Add a pair of directories into syncing
    Add { dir_a: String, dir_b: String },

    /// Remove a pair of directories from sync list
    Remove { pair_id: String },

    /// List the syncing pairs and their pair ids
    List,

    /// Pause syncing
    Pause {
        #[arg(required = true)]
        target: String, // pairId 或 --all
    },

    /// Resume syncing
    Resume {
        #[arg(required = true)]
        target: String,
    },

    /// Restart sync pairs
    Restart {
        #[arg(required = true)]
        target: String,
    },

    /// Show working status
    Status,

    /// Print logs
    Log {
        /// Specific pair id or `--all`
        #[arg(default_value = "--all")]
        target: String,

        /// Output log file (default: stdout)
        #[arg(long)]
        output: Option<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Service {
    Start,
    Stop,
    Enable,
    Disable,
    Restart,
}

// ===========================
// ======== UDS utils ========
// ===========================

// THIS SOCKET IS HARDCODED
const SOCKET_DIR: &str = "/run/synchron";
const SOCKET_PATH: &str = "/run/synchron/manager.sock";

#[derive(Debug, Error)]
pub enum UdsSetupError {
    #[error("create dir {0} failed: {1}")]
    CreateDir(&'static str, #[source] std::io::Error),

    #[error("stat path {0} failed: {1}")]
    Stat(&'static str, #[source] std::io::Error),

    #[error("chmod dir {0} failed: {1}")]
    ChmodDir(&'static str, #[source] std::io::Error),

    #[error("remove stale socket {0} failed: {1}")]
    RemoveStale(&'static str, #[source] std::io::Error),
}

// ============ Provide the UDS path ============

/// Prepare the UDS path for system services:
/// 1) Ensure the `/run/synchron` directory exists with permission 0700;
/// 2) If a stale socket with the same name exists, remove it first;
/// 3) Return a fixed socket PathBuf (the actual binding is done by the caller).
pub async fn handle_uds() -> Result<PathBuf, UdsSetupError> {
    // 1) Ensure the directory exists
    fs::create_dir_all(SOCKET_DIR)
        .await
        .map_err(|e| UdsSetupError::CreateDir(SOCKET_DIR, e))?;

    // 2) Set directory permission to 0700 (accessible only by the service user)
    let meta = fs::metadata(SOCKET_DIR)
        .await
        .map_err(|e| UdsSetupError::Stat(SOCKET_DIR, e))?;
    let perms = meta.permissions();
    if perms.mode() & 0o777 != 0o700 {
        let mut p = perms;
        p.set_mode(0o700);
        fs::set_permissions(SOCKET_DIR, p)
            .await
            .map_err(|e| UdsSetupError::ChmodDir(SOCKET_DIR, e))?;
    }

    // 3) Remove any leftover old socket file
    if fs::metadata(SOCKET_PATH).await.is_ok() {
        fs::remove_file(SOCKET_PATH)
            .await
            .map_err(|e| UdsSetupError::RemoveStale(SOCKET_PATH, e))?;
    }

    // 4) Return the fixed path (the caller will then execute UnixListener::bind(SOCKET_PATH)
    //    and set the 0600 permission by itself)
    Ok(PathBuf::from(SOCKET_PATH))
}

// =======================================================
// ======== Establish UDS connection with Manager ========
// =======================================================

#[derive(Debug, Error)]
pub enum UdsConnectError {
    #[error("failed to connect to manager socket {0}: {1}")]
    Connect(String, #[source] std::io::Error),
}

pub async fn connect_uds(sock_path: &Path) -> Result<UnixStream, UdsConnectError> {
    UnixStream::connect(sock_path)
        .await
        .map_err(|e| UdsConnectError::Connect(sock_path.display().to_string(), e))
}

async fn write_frame<W: AsyncWriteExt + Unpin>(w: &mut W, payload: &[u8]) -> io::Result<()> {
    let len = (payload.len() as u32).to_be_bytes();
    w.write_all(&len).await?;
    w.write_all(payload).await?;
    w.flush().await
}
async fn read_frame<R: AsyncReadExt + Unpin>(r: &mut R) -> io::Result<Vec<u8>> {
    let mut len = [0u8; 4];
    r.read_exact(&mut len).await?;
    let n = u32::from_be_bytes(len) as usize;
    let mut buf = vec![0u8; n];
    r.read_exact(&mut buf).await?;
    Ok(buf)
}

// ==========================================================
// ========== payload protocol (the simplest json) ==========
// ==========================================================

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    #[default]
    One,
    All,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum Mode {
    Bi,
    A2b,
    B2a,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ConflictPolicy {
    Ours,
    Theirs,
    Manual,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(serde::Serialize)]
struct Request {
    op: &'static str,
    id: u64,
    ts: String,
    #[serde(flatten)]
    params: Params,
}

#[derive(Serialize, Deserialize)]
enum Params {
    // pair.*
    PairAdd(PairAddParams),
    PairRemove {
        pair_id: String,
        #[serde(default)]
        purge_state: bool,
    },
    PairList,
    PairPause {
        scope: Scope,
        #[serde(default)]
        pair_id: Option<String>,
    },
    PairResume {
        scope: Scope,
        #[serde(default)]
        pair_id: Option<String>,
    },
    PairRestart {
        scope: Scope,
        #[serde(default)]
        pair_id: Option<String>,
    },

    // service.*
    ServiceStatus {
        #[serde(default)]
        detail: Option<String>,
    }, // "summary"|"full"

    // logs.*
    LogsTail(LogsTailParams),
}

#[derive(Serialize, Deserialize)]
pub struct PairAddParams {
    pub dir_a: PathBuf,
    pub dir_b: PathBuf,
    #[serde(default = "default_mode")]
    pub mode: Mode,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub include: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default)]
    pub bandwidth_mb: Option<u32>,
    #[serde(default)]
    pub max_inflight: Option<u32>,
    #[serde(default = "default_conflict_policy")]
    pub conflict_policy: ConflictPolicy,
}
fn default_mode() -> Mode {
    Mode::Bi
}
fn default_conflict_policy() -> ConflictPolicy {
    ConflictPolicy::Manual
}

#[derive(Serialize, Deserialize)]
pub struct LogsTailParams {
    #[serde(default)]
    pub scope: Scope, // 默认 One；若 All, 可不填 pair_id
    #[serde(default)]
    pub pair_id: Option<String>,
    #[serde(default)]
    pub since: Option<String>, // 支持 "1h" 或 RFC3339；也可换成自定义新类型
    #[serde(default)]
    pub level: Option<LogLevel>,
    #[serde(default)]
    pub follow: bool,
}

#[derive(serde::Serialize)]
struct MetaLite<'a> {
    root: &'a str,
    side: &'a str, // "A" / "B"
}

// ============================================
// ======== Send and recieve UDS utils ========
// ============================================

fn now_rfc3339() -> String {
    time::OffsetDateTime::now_utc().to_string()
}

fn next_req_id() -> u64 {
    // 最简单的：基于时间戳毫秒。够用；若需要，可换成 AtomicU64 递增。
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    (ms & u128::from(u64::MAX)) as u64
}

async fn send_json<W: AsyncWriteExt + Unpin>(w: &mut W, v: &Value) -> io::Result<()> {
    let bytes = serde_json::to_vec(v).expect("serialize request");
    write_frame(w, &bytes).await
}

async fn recv_json<R: AsyncReadExt + Unpin>(r: &mut R) -> io::Result<Value> {
    let bytes = read_frame(r).await?;
    let v: Value = serde_json::from_slice(&bytes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("invalid JSON: {e}")))?;
    Ok(v)
}

// 统一解析响应：要求 { "ok": bool, "error": null|{...}, "data": {...}, "request_id": ... }
fn unwrap_ok(resp: Value) -> io::Result<Value> {
    let ok = resp.get("ok").and_then(|b| b.as_bool()).unwrap_or(false);
    if ok {
        return Ok(resp.get("data").cloned().unwrap_or(Value::Null));
    }
    let code = resp
        .pointer("/error/code")
        .and_then(|x| x.as_str())
        .unwrap_or("UNKNOWN");
    let msg = resp
        .pointer("/error/message")
        .and_then(|x| x.as_str())
        .unwrap_or("unknown error");
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("{code}: {msg}"),
    ))
}

// ==================================================
// ========== HERE START THE MAIN FUNCTION ==========
// ==================================================
#[tokio::main]
async fn main() {
    let args = Args::parse();

    let sock_path = match handle_uds().await {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to prepare UDS socket: {e}");
            std::process::exit(1);
        }
    };

    let stream = match connect_uds(&sock_path).await {
        Ok(stream) => stream,
        Err(e) => {
            eprintln!("Connection failed: {e}");
            std::process::exit(1);
        }
    };

    let (mut r, mut w) = stream.into_split();

    let code: i32 = match args.action {
        Action::Service { service } => handle_service(service).await,

        Action::Add { dir_a, dir_b } => {
            let req = serde_json::json!({
                "op": "pair.add",
                "id": next_req_id(),
                "ts": now_rfc3339(),
                "params": {
                    "dir_a": dir_a,
                    "dir_b": dir_b,
                    "mode": "bi",
                    "include": [],
                    "exclude": [],
                    "conflict_policy": "manual"
                }
            });

            // 发送 + 等响应
            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                1
            } else {
                match recv_json(&mut r).await.and_then(unwrap_ok) {
                    Ok(data) => {
                        let pair_id = data
                            .get("pair_id")
                            .and_then(|x| x.as_str())
                            .unwrap_or("<unknown>");
                        println!("Added pair: id={pair_id}");
                        0
                    }
                    Err(e) => {
                        eprintln!("add failed: {e}");
                        2
                    }
                }
            }
        }

        Action::Remove { pair_id } => {
            let req = serde_json::json!({
                "op": "pair.remove",
                "request_id": next_req_id(),
                "ts": now_rfc3339(),
                "params": { "pair_id": pair_id, "purge_state": false }
            });

            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                1
            } else {
                match recv_json(&mut r).await.and_then(unwrap_ok) {
                    Ok(_) => {
                        println!("Removed: {pair_id}");
                        0
                    }
                    Err(e) => {
                        eprintln!("remove failed: {e}");
                        2
                    }
                }
            }
        }

        Action::List => {
            let req = serde_json::json!({
                "op": "pair.list",
                "id": next_req_id(),
                "ts": now_rfc3339(),
                "params": {}
            });

            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                1
            } else {
                match recv_json(&mut r).await.and_then(unwrap_ok) {
                    Ok(data) => {
                        if let Some(pairs) = data.get("pairs").and_then(|x| x.as_array()) {
                            if pairs.is_empty() {
                                println!("No pairs.");
                            } else {
                                for p in pairs {
                                    let id =
                                        p.get("pair_id").and_then(|x| x.as_str()).unwrap_or("?");
                                    let a = p.get("dir_a").and_then(|x| x.as_str()).unwrap_or("?");
                                    let b = p.get("dir_b").and_then(|x| x.as_str()).unwrap_or("?");
                                    let st = p
                                        .get("state")
                                        .and_then(|x| x.as_str())
                                        .unwrap_or("unknown");
                                    println!("{id}\t{st}\t{a}\t<->\t{b}");
                                }
                            }
                            0
                        } else {
                            eprintln!("bad response: missing pairs");
                            3
                        }
                    }
                    Err(e) => {
                        eprintln!("list failed: {e}");
                        2
                    }
                }
            }
        }

        Action::Pause { target } => {
            let (scope, pid) = if target == "--all" {
                ("all", serde_json::Value::Null)
            } else {
                ("one", serde_json::Value::String(target.clone()))
            };

            let req = serde_json::json!({
                "api": "synchron.v1",
                "op": "pair.pause",
                "request_id": next_req_id(),
                "ts": now_rfc3339(),
                "params": { "scope": scope, "pair_id": pid }
            });

            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                1
            } else {
                match recv_json(&mut r).await.and_then(unwrap_ok) {
                    Ok(data) => {
                        let affected = data.get("affected").cloned().unwrap_or(Value::Null);
                        println!("Paused: {affected}");
                        0
                    }
                    Err(e) => {
                        eprintln!("pause failed: {e}");
                        2
                    }
                }
            }
        }

        Action::Resume { target } => {
            let (scope, pid) = if target == "--all" {
                ("all", serde_json::Value::Null)
            } else {
                ("one", serde_json::Value::String(target.clone()))
            };

            let req = serde_json::json!({
                "api": "synchron.v1",
                "op": "pair.resume",
                "request_id": next_req_id(),
                "ts": now_rfc3339(),
                "params": { "scope": scope, "pair_id": pid }
            });

            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                1
            } else {
                match recv_json(&mut r).await.and_then(unwrap_ok) {
                    Ok(data) => {
                        let affected = data.get("affected").cloned().unwrap_or(Value::Null);
                        println!("Resumed: {affected}");
                        0
                    }
                    Err(e) => {
                        eprintln!("resume failed: {e}");
                        2
                    }
                }
            }
        }

        Action::Restart { target } => {
            let (scope, pid) = if target == "--all" {
                ("all", serde_json::Value::Null)
            } else {
                ("one", serde_json::Value::String(target.clone()))
            };

            let req = serde_json::json!({
                "api": "synchron.v1",
                "op": "pair.restart",
                "request_id": next_req_id(),
                "ts": now_rfc3339(),
                "params": { "scope": scope, "pair_id": pid }
            });

            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                1
            } else {
                match recv_json(&mut r).await.and_then(unwrap_ok) {
                    Ok(data) => {
                        let affected = data.get("affected").cloned().unwrap_or(Value::Null);
                        println!("Restarted: {affected}");
                        0
                    }
                    Err(e) => {
                        eprintln!("restart failed: {e}");
                        2
                    }
                }
            }
        }

        Action::Status => {
            let req = serde_json::json!({
                "api": "synchron.v1",
                "op": "service.status",
                "request_id": next_req_id(),
                "ts": now_rfc3339(),
                "params": { "detail": "summary" }
            });

            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                1
            } else {
                match recv_json(&mut r).await.and_then(unwrap_ok) {
                    Ok(data) => {
                        // 简要打印
                        if let Some(svc) = data.get("service") {
                            let ver = svc.get("version").and_then(|x| x.as_str()).unwrap_or("?");
                            let up = svc.get("uptime_sec").and_then(|x| x.as_u64()).unwrap_or(0);
                            println!("synchron {ver}, uptime {up}s");
                        }
                        if let Some(pairs) = data.get("pairs").and_then(|x| x.as_array()) {
                            for p in pairs {
                                let id = p.get("pair_id").and_then(|x| x.as_str()).unwrap_or("?");
                                let st = p.get("state").and_then(|x| x.as_str()).unwrap_or("?");
                                println!("  pair {id}: {st}");
                            }
                        }
                        0
                    }
                    Err(e) => {
                        eprintln!("status failed: {e}");
                        2
                    }
                }
            }
        }

        Action::Log { target, output } => 'log_branch: {
            let (scope, pid) = if target == "--all" {
                ("all", serde_json::Value::Null)
            } else {
                ("one", serde_json::Value::String(target.clone()))
            };

            let req = serde_json::json!({
                "api": "synchron.v1",
                "op": "logs.tail",
                "request_id": next_req_id(),
                "ts": now_rfc3339(),
                "params": {
                    "scope": scope,
                    "pair_id": pid,
                    "since": "1h",
                    "level": "info",
                    "follow": true,
                    "format": "text"
                }
            });

            if let Err(e) = send_json(&mut w, &req).await {
                eprintln!("send failed: {e}");
                break 'log_branch 1;
            }

            // 打开输出（若有）
            let mut file = if let Some(path) = output {
                match tokio::fs::File::create(&path).await {
                    Ok(f) => Some(f),
                    Err(e) => {
                        eprintln!("open output failed: {e}");
                        break 'log_branch 4; // 提前结束这个分支，返回退出码 4
                    }
                }
            } else {
                None
            };

            // 连续读取事件
            loop {
                match recv_json(&mut r).await {
                    Ok(v) => {
                        if v.get("type").and_then(|x| x.as_str()) == Some("log.eof") {
                            break;
                        }
                        let evt = if v.get("type").is_some() {
                            v.clone()
                        } else {
                            v.get("data").cloned().unwrap_or(v.clone())
                        };

                        if let (Some(ts), Some(level), Some(msg)) = (
                            evt.get("ts").and_then(|x| x.as_str()),
                            evt.get("level").and_then(|x| x.as_str()),
                            evt.get("msg").and_then(|x| x.as_str()),
                        ) {
                            let line = format!("{ts} {level:<5} {msg}\n");
                            if let Some(f) = file.as_mut() {
                                if let Err(e) =
                                    tokio::io::AsyncWriteExt::write_all(f, line.as_bytes()).await
                                {
                                    eprintln!("write file failed: {e}");
                                    break;
                                }
                            } else {
                                print!("{line}");
                            }
                        } else {
                            let s = evt.to_string();
                            if let Some(f) = file.as_mut() {
                                if tokio::io::AsyncWriteExt::write_all(f, s.as_bytes())
                                    .await
                                    .is_err()
                                    || tokio::io::AsyncWriteExt::write_all(f, b"\n").await.is_err()
                                {
                                    eprintln!("write file failed");
                                    break;
                                }
                            } else {
                                println!("{s}");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("log stream end: {e}");
                        break;
                    }
                }
            }
            0
        }
    };

    std::process::exit(code);
}
