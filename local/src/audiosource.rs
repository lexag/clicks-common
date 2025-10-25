use crate::{beatstate::BeatState, playbackstate::PlaybackState};
use mem::smpte::TimecodeInstant;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceState {
    BeatStatus(BeatState),
    TimeStatus(TimecodeInstant),
    PlaybackStatus(PlaybackState),
}
