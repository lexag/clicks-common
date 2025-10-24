use mem::str::String8;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Heartbeat {
    pub common_version: String8,
    pub system_version: String8,
    pub system_time: u64,
    pub cpu_use_audio: f32,
    pub process_freq_main: usize,
}
