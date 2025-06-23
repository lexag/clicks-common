use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{cue::Cue, show::Show};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ControlCommand {
    TransportStart,
    TransportStop,
    TransportZero,
    LoadCue(Cue),
    LoadCueByIndex(usize),
    LoadCueFromSelfIndex,
    LoadNextCue,
    LoadPreviousCue,
    LoadShow(Show),
    DumpStatus,
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
