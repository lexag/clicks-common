use mem::network::SubscriberInfo;

/// Wrapper for network status values
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy)]
pub struct NetworkStatus {
    /// List of network subscribers
    /// FIXME: Limited at 32 because serde... see other comment somewhere about this
    pub subscribers: [Option<SubscriberInfo>; 32],
}
