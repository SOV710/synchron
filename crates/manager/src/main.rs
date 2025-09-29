use std::path::PathBuf;
use std::process;
use synchron_utils::{create_uds, ensure_uds};

use clap::Parser;
use tokio;

/// Synchron-manager -- synchron's daemon
/// Supports --config (optional), plus auto-provided --help / --version.
#[derive(Parser, Debug)]
#[command(name = "synchron-manager", version)]
pub struct Args {
    /// Path to config file
    #[arg(long, value_name = "PATH")]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let socket_path = ensure_uds(PathBuf::from("/run/synchron/manager-control.sock"))
        .unwrap_or_else(|e| {
            eprintln!("Failed to prepare socket path: {}", e);
            std::process::exit(1);
        });
    let path = PathBuf::from(socket_path);
    let listener = match create_uds(path.clone()).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!(
                "Failed to create UDS at '{}': {} (kind: {:?})",
                path.display(),
                e,
                e.kind()
            );
            process::exit(1);
        }
    };
}
