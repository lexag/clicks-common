use mem::network::SubscriberInfo;

/// Wrapper for network status values
#[derive(Debug, Default, Clone, Copy)]
pub struct NetworkStatus {
    /// List of network subscribers
    pub subscribers: [Option<SubscriberInfo>; 32],
}
