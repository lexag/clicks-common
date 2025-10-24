use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlaybackState {
    pub clip_idx: u16,
    pub current_sample: i32,
    pub playing: bool,
    pub clips: [u16; 16],
}
