use serde::{Deserialize, Serialize};

use crate::cue::Cue;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeatStatus {
    pub beat_idx: usize,
    pub next_beat_idx: usize,
    pub us_to_next: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeStatus {
    pub h: usize,
    pub m: usize,
    pub s: usize,
    pub f: usize,
    pub fp: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceStatus {
    BeatStatus(BeatStatus),
    TimeStatus(TimeStatus),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStatus {
    pub sources: Vec<AudioSourceStatus>,
    pub running: bool,
    pub beat_idx: usize,
    pub next_beat_idx: usize,
    pub us_to_next_beat: usize,
    pub h: usize,
    pub m: usize,
    pub s: usize,
    pub cue: Cue,
}
