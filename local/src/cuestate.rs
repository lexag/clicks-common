use cue::Cue;

/// Status of the current cue
#[derive(Clone, Debug, Default, Copy)]
pub struct CueState {
    /// Cue idx of this cue in the show
    pub cue_idx: u16,
    /// Cue data itself
    pub cue: Cue,
}
