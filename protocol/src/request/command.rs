use core::fmt;
use core::result;
use serde::{Deserialize, Serialize};

use event::event::JumpModeChange;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum ControlAction {
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

impl fmt::Display for ControlAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            ControlAction::TransportStart => write!(f, "TransportStart"),
            ControlAction::TransportStop => write!(f, "TransportStop"),
            ControlAction::TransportZero => write!(f, "TransportZero"),
            ControlAction::TransportSeekBeat(..) => write!(f, "TransportSeekBeat"),
            ControlAction::TransportJumpBeat(..) => write!(f, "TransportJumpBeat"),
            ControlAction::LoadCueByIndex(..) => write!(f, "LoadCueByIndex"),
            ControlAction::LoadCueFromSelfIndex => write!(f, "LoadCueFromSelfIndex"),
            ControlAction::LoadNextCue => write!(f, "LoadNextCue"),
            ControlAction::LoadPreviousCue => write!(f, "LoadPreviousCue"),
            ControlAction::DumpStatus => write!(f, "DumpStatus"),
            ControlAction::SetChannelGain(..) => write!(f, "SetChannelGain"),
            ControlAction::SetChannelMute(..) => write!(f, "SetChannelMute"),
            ControlAction::ChangeJumpMode(..) => write!(f, "ChangeJumpMode"),
            ControlAction::ChangePlayrate(..) => write!(f, "ChangePlayrate"),
        }
    }
}
