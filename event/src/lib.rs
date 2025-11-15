#![warn(missing_docs)]
#![no_std]
//! Definitions (and supporting data formats) for Events, which happen on a specific beat in a cue
mod event;
pub use event::Event;
pub use event::EventDescription;
pub use event::JumpModeChange;
pub use event::JumpRequirement;
pub use event::PauseEventBehaviour;

mod eventcursor;
pub use eventcursor::EventCursor;

mod table;
pub use table::EventTable;
