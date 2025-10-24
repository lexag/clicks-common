use crate::{
    audiosource::AudioSourceState, beatstate::BeatState, cuestate::CueState,
    jackstatus::JACKStatus, networkstatus::NetworkStatus, playbackstate::PlaybackState,
    transportstate::TransportState,
};
use cue::Show;
use time::timecode::TimecodeInstant;

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
