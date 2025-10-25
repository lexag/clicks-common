#![warn(missing_docs)]
//! This crate contains common functionality and data formats shared between the ClicKS core and
//! client components

pub use beat;
pub use cue;
/// Event types
pub use event;
/// Definitions pertaining to local core storage that other clients might still have use for, such
/// as configuration and status
pub use local;
/// Low-level data representation formats
pub use mem;
/// Defines the data format for the network protocol used to communicate between
/// between the ClicKS core and ClicKS clients.
pub use protocol;
