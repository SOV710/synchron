#![allow(dead_code)]
use clap::ValueEnum;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::sync::{broadcast, mpsc};
use tokio::task::JoinHandle;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, ValueEnum)]
pub enum Side {
    A = 0,
    B = 1,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Metadata {
    pub root: PathBuf,
    pub side: Side,
}

/// dispacther and manager

pub enum Action {
    Write,
    Delete,
    Rename,
}

pub struct RawEvent {}
pub struct NormalizedEvent {}
pub struct CoalescedEvent {}

/// Worker and threads

pub struct Event {
    metadata: Metadata,
    path: PathBuf,
    action: Action,
    pub ts: SystemTime,
}

pub struct Handle {
    pub rx: mpsc::Receiver<Event>,
    stop: broadcast::Sender<()>,
    join: JoinHandle<()>,
}
