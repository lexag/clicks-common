use mem::str::String32;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ChannelAssignment {
    L,
    R,
    #[default]
    Mono,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(default)]
pub struct ChannelConfiguration {
    pub name: String32,
    pub channel_assignment: ChannelAssignment,
    pub gain: f32,
}
