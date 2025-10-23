extern crate serde_big_array;

pub mod command;
pub mod config;
pub mod control;
pub mod network;
pub mod status;
pub mod time;
pub mod timecode;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
