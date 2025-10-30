use mem::str::String32;
use serde::{Deserialize, Serialize};

/// Cosmetic selector value for whether a channel is mono or stereo, in which case which side
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ChannelAssignment {
    /// This channel is the left half of a stereo channel pair
    L,
    /// This channel is the right half of a stereo channel pair
    R,
    /// This channel is a mono channel
    #[default]
    Mono,
}

/// Configuration values for audio channels
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(default)]
pub struct ChannelConfiguration {
    /// Name of this channel
    pub name: String32,
    /// Mono or stereo assignment
    pub channel_assignment: ChannelAssignment,
    /// Digital gain value of this channel in dB. 0 is unchanged volume.
    pub gain: f32,
}
