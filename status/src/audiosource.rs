use crate::{beatstate::BeatState, playbackstate::PlaybackState};
use serde::{Deserialize, Serialize};
use time::timecode::TimecodeInstant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceState {
    BeatStatus(BeatState),
    TimeStatus(TimecodeInstant),
    PlaybackStatus(PlaybackState),
}
