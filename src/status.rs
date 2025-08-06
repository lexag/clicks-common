use serde::{Deserialize, Serialize};

use crate::{
    config::SystemConfiguration,
    cue::{Beat, Cue},
    network::{Heartbeat, JACKStatus, NetworkStatus},
    show::Show,
    timecode::TimecodeInstant,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaybackState {
    pub clip_idx: usize,
    pub current_sample: i32,
    pub playing: bool,
    pub clips: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BeatState {
    pub beat_idx: usize,
    pub next_beat_idx: usize,
    pub beat: Beat,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CueState {
    pub cue_idx: usize,
    pub cue: Cue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceState {
    BeatStatus(BeatState),
    TimeStatus(TimecodeInstant),
    PlaybackStatus(PlaybackState),
    None,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TransportState {
    pub us_to_next_beat: usize,
    pub running: bool,
}

//#[derive(Default, Debug, Clone, Serialize, Deserialize)]
//pub struct ProcessStatus {
//    pub sources: Vec<AudioSourceState>,
//    pub running: bool,
//    pub beat_idx: usize,
//    pub next_beat_idx: usize,
//    pub us_to_next_beat: usize,
//    pub time: TimecodeInstant,
//    pub cue_idx: usize,
//}

#[derive(Default, Clone, Debug)]
pub struct CombinedStatus {
    pub sources: Vec<AudioSourceState>,
    pub transport: TransportState,
    pub cue: CueState,
    pub show: Show,
    pub network_status: NetworkStatus,
    pub jack_status: JACKStatus,
}

impl CombinedStatus {
    pub fn beat_state(&self) -> BeatState {
        if let AudioSourceState::BeatStatus(state) = &self.sources[0] {
            state.clone()
        } else {
            panic!("Metronome is not in slot 0")
        }
    }
    pub fn time_state(&self) -> TimecodeInstant {
        if let AudioSourceState::TimeStatus(state) = &self.sources[1] {
            state.clone()
        } else {
            panic!("Timecode is not in slot 1")
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NotificationKind {
    TransportChanged,
    BeatChanged,
    CueChanged,
    ShowChanged,
    NetworkChanged,
    JACKStateChanged,
    ConfigurationChanged,
    ShutdownOccured,
    Heartbeat,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Notification {
    TransportChanged(TransportState),
    BeatChanged(BeatState),
    CueChanged(CueState),
    ShowChanged(Show),
    NetworkChanged(NetworkStatus),
    JACKStateChanged(JACKStatus),
    ConfigurationChanged(SystemConfiguration),
    ShutdownOccured,
    Heartbeat(Heartbeat),
}

impl Notification {
    pub fn to_kind(&self) -> NotificationKind {
        match self {
            Self::TransportChanged(..) => NotificationKind::TransportChanged,
            Self::BeatChanged(..) => NotificationKind::BeatChanged,
            Self::CueChanged(..) => NotificationKind::JACKStateChanged,
            Self::ShowChanged(..) => NotificationKind::CueChanged,
            Self::NetworkChanged(..) => NotificationKind::ShowChanged,
            Self::JACKStateChanged(..) => NotificationKind::NetworkChanged,
            Self::ConfigurationChanged(..) => NotificationKind::ConfigurationChanged,
            Self::ShutdownOccured => NotificationKind::ShutdownOccured,
            Self::Heartbeat(..) => NotificationKind::Heartbeat,
        }
    }
}
