use std::fmt::{Debug, Display};

use serde::{Deserialize, Serialize};

use crate::{
    command::ControlCommand,
    config::SystemConfiguration,
    cue::Cue,
    network::{ConnectionInfo, JACKStatus, NetworkStatus, SubscriberInfo},
    show::Show,
    timecode::TimecodeInstant,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaybackStatus {
    pub clip_idx: usize,
    pub current_sample: i32,
    pub playing: bool,
    pub clips: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeatStatus {
    pub beat_idx: usize,
    pub next_beat_idx: usize,
    pub us_to_next: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceStatus {
    BeatStatus(BeatStatus),
    TimeStatus(TimecodeInstant),
    PlaybackStatus(PlaybackStatus),
    None,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStatus {
    pub sources: Vec<AudioSourceStatus>,
    pub running: bool,
    pub beat_idx: usize,
    pub next_beat_idx: usize,
    pub us_to_next_beat: usize,
    pub time: TimecodeInstant,
    pub system_time_us: u64,
    pub cue_idx: usize,
    pub cpu_use: f32,
}

#[derive(Default)]
pub struct CombinedStatus {
    pub process_status: ProcessStatus,
    pub cue: Cue,
    pub show: Show,
    pub network_status: NetworkStatus,
    pub jack_status: JACKStatus,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusMessageKind {
    ProcessStatus,
    CueStatus,
    ShowStatus,
    NetworkStatus,
    JACKStatus,
    ConfigurationStatus,
    Shutdown,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum StatusMessage {
    ProcessStatus(ProcessStatus),
    CueStatus(Cue),
    ShowStatus(Show),
    NetworkStatus(NetworkStatus),
    JACKStatus(JACKStatus),
    ConfigurationStatus(SystemConfiguration),
    Shutdown,
}

impl StatusMessage {
    pub fn to_kind(&self) -> StatusMessageKind {
        match self {
            Self::ProcessStatus(..) => StatusMessageKind::ProcessStatus,
            Self::JACKStatus(..) => StatusMessageKind::JACKStatus,
            Self::CueStatus(..) => StatusMessageKind::CueStatus,
            Self::ShowStatus(..) => StatusMessageKind::ShowStatus,
            Self::NetworkStatus(..) => StatusMessageKind::NetworkStatus,
            Self::Shutdown => StatusMessageKind::Shutdown,
            Self::ConfigurationStatus(..) => StatusMessageKind::ConfigurationStatus,
        }
    }
}

impl Debug for StatusMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            StatusMessage::ProcessStatus(status) => {
                write!(f, "{status:?}")
            }
            StatusMessage::CueStatus(status) => write!(f, "{status:?}"),
            StatusMessage::ConfigurationStatus(status) => {
                write!(f, "{status:?}")
            }
            StatusMessage::ShowStatus(status) => write!(f, "{status:?}"),
            StatusMessage::NetworkStatus(status) => {
                write!(f, "{status:?}")
            }
            StatusMessage::JACKStatus(status) => write!(f, "{status:?}"),
            _ => write!(f, "Unimplemented representation."),
        }
    }
}
