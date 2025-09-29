use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use tokio::net::UnixListener;

pub fn ensure_uds(path: PathBuf) -> std::io::Result<PathBuf> {
    let dir = path
        .parent()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid path"))?;

    fs::create_dir_all(dir)?;
    fs::set_permissions(dir, fs::Permissions::from_mode(0o700))?;

    let _ = fs::remove_file(&path);

    Ok(path)
}

pub async fn create_uds(path: PathBuf) -> std::io::Result<UnixListener> {
    let listener = UnixListener::bind(&path)?;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;

    Ok(listener)
}
