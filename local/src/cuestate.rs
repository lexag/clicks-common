use cue::Cue;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CueState {
    pub cue_idx: u16,
    pub cue: Cue,
}
