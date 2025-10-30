use mem::smpte::TimecodeInstant;
use serde::{Deserialize, Serialize};

/// Status of current transport location and state
#[derive(Clone, Debug, Serialize, Deserialize, Copy)]
pub struct TransportState {
    /// Time left to next beat in us
    pub us_to_next_beat: u64,
    /// Is transport currently running, i.e. is location changing
    pub running: bool,
    /// VLT state
    pub vlt: bool,
    /// current LTC timestamp
    pub ltc: TimecodeInstant,
    /// Playrate in percent
    pub playrate_percent: u16,
}

impl Default for TransportState {
    fn default() -> Self {
        Self {
            us_to_next_beat: 0,
            running: false,
            vlt: false,
            ltc: TimecodeInstant::default(),
            playrate_percent: 100,
        }
    }
}
