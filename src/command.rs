use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::cue::Cue;

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ControlCommand {
    TransportStart,
    TransportStop,
    TransportZero,
    NotifySubscribers,
    LoadCue(Cue),
    Shutdown,
    LoadCueByIndex(usize),
    LoadCueFromSelfIndex,
    LoadNextCue,
    LoadPreviousCue,
}

#[derive(Debug)]
pub enum CommandError {
    UnknownCommand,
    IsRunning,
    IsNotRunning,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::UnknownCommand => write!(f, "Unknown command"),
            _ => {
                write!(f, "")
            }
        }
    }
}

impl Error for CommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}
