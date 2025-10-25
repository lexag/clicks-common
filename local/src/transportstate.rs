use mem::smpte::TimecodeInstant;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransportState {
    pub us_to_next_beat: u16,
    pub running: bool,
    pub vlt: bool,
    pub ltc: TimecodeInstant,
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
