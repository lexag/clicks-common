use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::cue::Cue;

#[derive(Clone, Deserialize, Serialize)]
pub enum ControlCommand {
    TransportStart,
    TransportStop,
    TransportZero,
    NotifySubscribers,
    LoadCue(Cue),
    Shutdown,
}

impl fmt::Debug for ControlCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ControlCommand::TransportZero => write!(f, "TransportZero"),
            ControlCommand::TransportStart => write!(f, "TransportStart"),
            ControlCommand::TransportStop => write!(f, "TransportStop"),
            ControlCommand::NotifySubscribers => write!(f, "NotifySubscribers"),
            ControlCommand::LoadCue(_cue) => write!(f, "LoadCue"),
            ControlCommand::Shutdown => write!(f, "Shutdown"),
        }
    }
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
