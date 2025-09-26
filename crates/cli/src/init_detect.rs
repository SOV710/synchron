use std::fs;
use std::path::PathBuf;
use std::process::Command;

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum InitSystem {
    Systemd,
    OpenRc,
    Runit,
    SysV,
    Upstart,
    BusyBoxInit,
    S6,
    Tini,
    DumbInit,
    Other(String),
    Unknown,
}

#[derive(Debug, Error)]
pub enum DetectError {
    #[error("failed to read /proc/1/comm: {0}")]
    ReadComm(#[from] std::io::Error),

    #[error("failed to read /proc/1/exe: {0}")]
    ReadExe(#[source] std::io::Error),

    #[error("this detector only works on Linux (/proc available)")]
    NotLinux,

    #[error("could not detect init system (insufficient data)")]
    Unknown,
}

fn read_proc1_comm() -> Result<String, DetectError> {
    let s = fs::read_to_string("/proc/1/comm").map_err(DetectError::ReadComm)?;
    Ok(s.trim().to_string())
}

fn read_proc1_exe() -> Result<PathBuf, DetectError> {
    let p = fs::read_link("/proc/1/exe").map_err(DetectError::ReadExe)?;
    Ok(p)
}

fn try_init_version() -> Option<String> {
    let out = Command::new("/sbin/init").arg("--version").output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).to_string())
    } else {
        None
    }
}

fn lower(s: &str) -> String {
    s.to_ascii_lowercase()
}

pub fn detect() -> Result<InitSystem, DetectError> {
    if !cfg!(target_os = "linux") {
        return Err(DetectError::NotLinux);
    }

    let comm = read_proc1_comm()?;
    let exe = read_proc1_exe()?;
    let exe_str = exe.to_string_lossy().to_string();

    let cname = lower(&comm);
    let cexe = lower(&exe_str);

    let guess = if cname.contains("systemd") || cexe.contains("systemd") {
        InitSystem::Systemd
    } else if cname.contains("openrc") || cexe.contains("openrc-init") {
        InitSystem::OpenRc
    } else if cname.contains("runit") || cexe.contains("runit") {
        InitSystem::Runit
    } else if cname == "s6-svscan" || cexe.contains("s6-svscan") {
        InitSystem::S6
    } else if cname.contains("tini") || cexe.contains("tini") {
        InitSystem::Tini
    } else if cname.contains("dumb-init") || cexe.contains("dumb-init") {
        InitSystem::DumbInit
    } else if cname == "init" || cexe.ends_with("/init") {
        if let Some(v) = try_init_version() {
            let vlow = lower(&v);
            if vlow.contains("sysv") || vlow.contains("sysvinit") {
                InitSystem::SysV
            } else if vlow.contains("upstart") {
                InitSystem::Upstart
            } else if vlow.contains("busybox") {
                InitSystem::BusyBoxInit
            } else {
                InitSystem::Other(format!("init ({v})"))
            }
        } else {
            InitSystem::Other("init".into())
        }
    } else if !cname.is_empty() {
        InitSystem::Other(comm.clone())
    } else {
        return Err(DetectError::Unknown);
    };

    Ok(guess)
}
