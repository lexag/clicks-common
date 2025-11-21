use mem::network::SubscriberInfo;

/// Wrapper for network status values
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, bincode::Encode, bincode::Decode)]
pub struct NetworkStatus {
    /// List of network subscribers
    pub subscribers: [Option<SubscriberInfo>; 32],
}
