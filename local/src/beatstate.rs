use beat::Beat;
use event::JumpModeChange;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BeatState {
    pub beat_idx: u16,
    pub next_beat_idx: u16,
    pub beat: Beat,
    pub requested_vlt_action: JumpModeChange,
}
