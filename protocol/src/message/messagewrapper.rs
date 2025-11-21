use crate::message::{LargeMessage, SmallMessage};

/// Wrapper type for both types of core -> client messages
#[allow(clippy::large_enum_variant)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, bincode::Encode, bincode::Decode)]
pub enum Message {
    /// Small, const-size, uC-friendly status updates
    Small(SmallMessage),
    /// Large dynamic information dump messages
    Large(LargeMessage),
}
