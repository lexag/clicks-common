use core::fmt;
use mem::str::String8;
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
pub struct Event {
    pub location: u16,
    pub event: Option<EventDescription>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            location: u16::MAX,
            event: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Ord, PartialOrd, Eq, Copy)]
pub enum EventDescription {
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

impl EventDescription {
    pub fn get_name(&self) -> &str {
        match self {
            EventDescription::JumpEvent { .. } => "Jump",
            EventDescription::TempoChangeEvent { .. } => "Tempo Change",
            EventDescription::GradualTempoChangeEvent { .. } => "Gradual Tempo Change",
            EventDescription::PlaybackEvent { .. } => "Playback",
            EventDescription::PlaybackStopEvent { .. } => "Playback Stop",
            EventDescription::TimecodeEvent { .. } => "Timecode",
            EventDescription::RehearsalMarkEvent { .. } => "Rehearsal Mark",
            EventDescription::PauseEvent { .. } => "Pause Event",
        }
    }
}
