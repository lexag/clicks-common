use serde::{Deserialize, Serialize};

use crate::{
    config::SystemConfiguration,
    cue::{Beat, Cue, JumpModeChange},
    network::{Heartbeat, JACKStatus, NetworkStatus},
    show::Show,
    timecode::TimecodeInstant,
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub clip_idx: u16,
    pub current_sample: i32,
    pub playing: bool,
    pub clips: [u16; 16],
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BeatState {
    pub beat_idx: u16,
    pub next_beat_idx: u16,
    pub beat: Beat,
    pub requested_vlt_action: JumpModeChange,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CueState {
    pub cue_idx: u16,
    pub cue: Cue,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceState {
    BeatStatus(BeatState),
    TimeStatus(TimecodeInstant),
    PlaybackStatus(PlaybackState),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransportState {
    pub us_to_next_beat: u16,
    pub running: bool,
    pub vlt: bool,
    pub ltc: TimecodeInstant,
    pub playrate_percent: u16,
}

impl Default for TransportState {
    fn default() -> Self {
        Self {
            us_to_next_beat: 0,
            running: false,
            vlt: false,
            ltc: TimecodeInstant::default(),
            playrate_percent: 100,
        }
    }
}

//#[derive(Default, Debug, Clone, Serialize, Deserialize)]
//pub struct ProcessStatus {
//    pub sources: Vec<AudioSourceState>,
//    pub running: bool,
//    pub beat_idx: u16,
//    pub next_beat_idx: u16,
//    pub us_to_next_beat: u16,
//    pub time: TimecodeInstant,
//    pub cue_idx: u16,
//}

#[derive(Clone, Debug)]
pub struct CombinedStatus {
    pub sources: [AudioSourceState; 32],
    pub transport: TransportState,
    pub cue: CueState,
    pub show: Show,
    pub network_status: NetworkStatus,
    pub jack_status: JACKStatus,
}

impl Default for CombinedStatus {
    fn default() -> Self {
        Self {
            // FIXME: uglyyyyyyyyyyyyyyyy
            sources: [
                AudioSourceState::BeatStatus(BeatState::default()),
                AudioSourceState::TimeStatus(TimecodeInstant::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
                AudioSourceState::PlaybackStatus(PlaybackState::default()),
            ],
            transport: TransportState::default(),
            cue: CueState::default(),
            show: Show::default(),
            network_status: NetworkStatus::default(),
            jack_status: JACKStatus::default(),
        }
    }
}

impl CombinedStatus {
    pub fn beat_state(&self) -> BeatState {
        if self.sources.is_empty() {
            return BeatState::default();
        }
        if let AudioSourceState::BeatStatus(state) = &self.sources[0] {
            state.clone()
        } else {
            panic!(
                "Metronome is not in slot 0. Slot 0 contains {:?}",
                &self.sources[0]
            )
        }
    }
    pub fn time_state(&self) -> TimecodeInstant {
        if self.sources.is_empty() {
            return TimecodeInstant::default();
        }
        if let AudioSourceState::TimeStatus(state) = &self.sources[1] {
            state.clone()
        } else {
            panic!(
                "Timecode is not in slot 1. Slot 1 contains {:?}",
                &self.sources[1]
            )
        }
    }
}

bitflags::bitflags! {
    #[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct NotificationKind: u8 {
        const TransportChanged = 0x01;
        const BeatChanged = 0x01;
        const CueChanged = 0x02;
        const ShowChanged = 0x04;
        const NetworkChanged = 0x08;
        const JACKStateChanged = 0x10;
        const ConfigurationChanged = 0x20;
        const ShutdownOccured = 0x40;
        const Heartbeat = 0x80;
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Notification {
    TransportChanged(TransportState),
    BeatChanged(BeatState),
    CueChanged(CueState),
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
            Self::NetworkChanged(..) => NotificationKind::ShowChanged,
            Self::JACKStateChanged(..) => NotificationKind::NetworkChanged,
            Self::ConfigurationChanged(..) => NotificationKind::ConfigurationChanged,
            Self::ShutdownOccured => NotificationKind::ShutdownOccured,
            Self::Heartbeat(..) => NotificationKind::Heartbeat,
        }
    }
}
