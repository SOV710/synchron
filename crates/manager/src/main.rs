use clap::Parser;
use synchron_common::{Metadata, Side};
use tokio;

#[derive(Parser, Debug)]
#[command(name = "synchron-manager", version)]
pub struct Args {
    /// a side directory
    #[arg(short)]
    a: Side,

    /// b side directory
    #[arg(short)]
    b: Side,
}

#[tokio::main]
async fn main() {}
