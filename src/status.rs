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
pub enum NotificationKind {
    TransportChanged,
    CueChanged,
    ShowChanged,
    NetworkChanged,
    JACKStateChanged,
    ConfigurationChanged,
    ShutdownOccured,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Notification {
    TransportChanged(ProcessStatus),
    CueChanged(Cue),
    ShowChanged(Show),
    NetworkChanged(NetworkStatus),
    JACKStateChanged(JACKStatus),
    ConfigurationChanged(SystemConfiguration),
    ShutdownOccured,
}

impl Notification {
    pub fn to_kind(&self) -> NotificationKind {
        match self {
            Self::TransportChanged(..) => NotificationKind::TransportChanged,
            Self::CueChanged(..) => NotificationKind::JACKStateChanged,
            Self::ShowChanged(..) => NotificationKind::CueChanged,
            Self::NetworkChanged(..) => NotificationKind::ShowChanged,
            Self::JACKStateChanged(..) => NotificationKind::NetworkChanged,
            Self::ConfigurationChanged(..) => NotificationKind::ConfigurationChanged,
            Self::ShutdownOccured => NotificationKind::ShutdownOccured,
        }
    }
}

impl Debug for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Notification::TransportChanged(status) => {
                write!(f, "{status:?}")
            }
            Notification::CueChanged(status) => write!(f, "{status:?}"),
            Notification::ConfigurationChanged(status) => {
                write!(f, "{status:?}")
            }
            Notification::ShowChanged(status) => write!(f, "{status:?}"),
            Notification::NetworkChanged(status) => {
                write!(f, "{status:?}")
            }
            Notification::JACKStateChanged(status) => write!(f, "{status:?}"),
            _ => write!(f, "Unimplemented representation."),
        }
    }
}
