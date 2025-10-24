use crate::str::String8;
use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, PartialOrd, Ord, Eq, Copy)]
pub enum JumpRequirement {
    JumpModeOn,
    JumpModeOff,
    None,
}

impl fmt::Display for JumpRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpRequirement::JumpModeOn => write!(f, "VLT On"),
            JumpRequirement::JumpModeOff => write!(f, "VLT Off"),
            JumpRequirement::None => write!(f, "None"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default, PartialOrd, Ord, Eq, Copy)]
pub enum JumpModeChange {
    SetOn,
    SetOff,
    Toggle,
    #[default]
    None,
}

impl fmt::Display for JumpModeChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpModeChange::SetOn => write!(f, "Set VLT"),
            JumpModeChange::SetOff => write!(f, "Trip VLT"),
            JumpModeChange::Toggle => write!(f, "Toggle VLT"),
            JumpModeChange::None => write!(f, "None"),
        }
    }
}
impl JumpModeChange {
    pub fn vlt(&self, vlt: bool) -> bool {
        match self {
            JumpModeChange::SetOn => true,
            JumpModeChange::SetOff => false,
            JumpModeChange::Toggle => !vlt,
            JumpModeChange::None => vlt,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, PartialOrd, Ord, Eq, Copy)]
pub enum PauseEventBehaviour {
    Hold,
    RestartBeat,
    RestartCue,
    NextCue,
    Jump { destination: u16 },
}

impl fmt::Display for PauseEventBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PauseEventBehaviour::Hold => {
                write!(f, "Hold")
            }
            PauseEventBehaviour::RestartBeat => {
                write!(f, "Restart beat")
            }
            PauseEventBehaviour::RestartCue => {
                write!(f, "Restart cue")
            }
            PauseEventBehaviour::NextCue => {
                write!(f, "Next cue")
            }
            PauseEventBehaviour::Jump { .. } => {
                write!(f, "Jump to beat")
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct BeatEventContainer {
    pub location: u16,
    pub event: Option<BeatEvent>,
}

impl Default for BeatEventContainer {
    fn default() -> Self {
        Self {
            location: u16::MAX,
            event: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Ord, PartialOrd, Eq, Copy)]
pub enum BeatEvent {
    JumpEvent {
        destination: u16,
        requirement: JumpRequirement,
        when_jumped: JumpModeChange,
        when_passed: JumpModeChange,
    },
    TempoChangeEvent {
        tempo: u16,
    },
    GradualTempoChangeEvent {
        start_tempo: u16,
        end_tempo: u16,
        length: u16,
    },
    PlaybackEvent {
        channel_idx: u16,
        clip_idx: u16,
        sample: i32,
    },
    PlaybackStopEvent {
        channel_idx: u16,
    },
    TimecodeEvent {
        h: u8,
        m: u8,
        s: u8,
        f: u8,
    },
    RehearsalMarkEvent {
        label: String8,
    },
    PauseEvent {
        behaviour: PauseEventBehaviour,
    },
}

impl BeatEvent {
    pub fn get_name(&self) -> &str {
        match self {
            BeatEvent::JumpEvent { .. } => "Jump",
            BeatEvent::TempoChangeEvent { .. } => "Tempo Change",
            BeatEvent::GradualTempoChangeEvent { .. } => "Gradual Tempo Change",
            BeatEvent::PlaybackEvent { .. } => "Playback",
            BeatEvent::PlaybackStopEvent { .. } => "Playback Stop",
            BeatEvent::TimecodeEvent { .. } => "Timecode",
            BeatEvent::RehearsalMarkEvent { .. } => "Rehearsal Mark",
            BeatEvent::PauseEvent { .. } => "Pause Event",
        }
    }
}
