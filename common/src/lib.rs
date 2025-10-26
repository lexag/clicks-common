#![warn(missing_docs)]
//! This crate contains common functionality and data formats shared between the ClicKS core and
//! client components

pub use beat;
pub use cue;
pub use event;
pub use local;
pub use mem;
/// Defines the data format for the network protocol used to communicate between
/// between the ClicKS core and ClicKS clients.
pub use protocol;
