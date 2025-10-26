use cue::Cue;
use serde::{Deserialize, Serialize};

/// Status of the current cue
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CueState {
    /// Cue idx of this cue in the show
    pub cue_idx: u16,
    /// Cue data itself
    pub cue: Cue,
}
