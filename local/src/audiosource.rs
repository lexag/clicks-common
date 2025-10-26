use crate::{beatstate::BeatState, playbackstate::PlaybackState};
use mem::smpte::TimecodeInstant;
use serde::{Deserialize, Serialize};

/// Wrapper type for the state of an audio source (audio channel)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceState {
    /// Metronome channel state
    BeatStatus(BeatState),
    /// Timecode channel state
    TimeStatus(TimecodeInstant),
    /// Playback channel state
    PlaybackStatus(PlaybackState),
}
