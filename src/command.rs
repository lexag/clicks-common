use std::fmt;
use std::{error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{cue::Cue, show::Show};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum ControlCommand {
    TransportStart,
    TransportStop,
    TransportZero,
    TransportSeekBeat(usize),
    TransportJumpBeat(usize),
    LoadCue(Cue),
    LoadCueByIndex(usize),
    LoadCueFromSelfIndex,
    LoadNextCue,
    LoadPreviousCue,
    LoadShow(Show),
    DumpStatus,
}

impl Display for ControlCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ControlCommand::TransportStart => write!(f, "TransportStart"),
            ControlCommand::TransportStop => write!(f, "TransportStop"),
            ControlCommand::TransportZero => write!(f, "TransportZero"),
            ControlCommand::TransportSeekBeat(..) => write!(f, "TransportSeekBeat"),
            ControlCommand::TransportJumpBeat(..) => write!(f, "TransportJumpBeat"),
            ControlCommand::LoadCue(..) => write!(f, "LoadCue"),
            ControlCommand::LoadCueByIndex(..) => write!(f, "LoadCueByIndex"),
            ControlCommand::LoadCueFromSelfIndex => write!(f, "LoadCueFromSelfIndex"),
            ControlCommand::LoadNextCue => write!(f, "LoadNextCue"),
            ControlCommand::LoadPreviousCue => write!(f, "LoadPreviousCue"),
            ControlCommand::LoadShow(..) => write!(f, "LoadShow"),
            ControlCommand::DumpStatus => write!(f, "DumpStatus"),
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
