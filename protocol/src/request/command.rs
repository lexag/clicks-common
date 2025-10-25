use core::fmt;
use core::result;
use serde::{Deserialize, Serialize};

use event::event::JumpModeChange;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum ControlCommand {
    TransportStart,
    TransportStop,
    TransportZero,
    TransportSeekBeat(usize),
    TransportJumpBeat(usize),
    LoadCueByIndex(usize),
    LoadCueFromSelfIndex,
    LoadNextCue,
    LoadPreviousCue,
    DumpStatus,
    SetChannelGain(usize, f32),
    SetChannelMute(usize, bool),
    ChangeJumpMode(JumpModeChange),
    ChangePlayrate(usize),
}

impl fmt::Display for ControlCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            ControlCommand::TransportStart => write!(f, "TransportStart"),
            ControlCommand::TransportStop => write!(f, "TransportStop"),
            ControlCommand::TransportZero => write!(f, "TransportZero"),
            ControlCommand::TransportSeekBeat(..) => write!(f, "TransportSeekBeat"),
            ControlCommand::TransportJumpBeat(..) => write!(f, "TransportJumpBeat"),
            ControlCommand::LoadCueByIndex(..) => write!(f, "LoadCueByIndex"),
            ControlCommand::LoadCueFromSelfIndex => write!(f, "LoadCueFromSelfIndex"),
            ControlCommand::LoadNextCue => write!(f, "LoadNextCue"),
            ControlCommand::LoadPreviousCue => write!(f, "LoadPreviousCue"),
            ControlCommand::DumpStatus => write!(f, "DumpStatus"),
            ControlCommand::SetChannelGain(..) => write!(f, "SetChannelGain"),
            ControlCommand::SetChannelMute(..) => write!(f, "SetChannelMute"),
            ControlCommand::ChangeJumpMode(..) => write!(f, "ChangeJumpMode"),
            ControlCommand::ChangePlayrate(..) => write!(f, "ChangePlayrate"),
        }
    }
}
