use crate::message::{LargeMessage, SmallMessage};

/// Wrapper type for both types of core -> client messages
#[allow(clippy::large_enum_variant)]
pub enum Message {
    /// Small, const-size, uC-friendly status updates
    Small(SmallMessage),
    /// Large dynamic information dump messages
    Large(LargeMessage),
}
