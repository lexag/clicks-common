use mem::network::SubscriberInfo;
use serde::{Deserialize, Serialize};

/// Wrapper for network status values
#[derive(Deserialize, Serialize, Debug, Default, Clone, Copy)]
pub struct NetworkStatus {
    /// List of network subscribers
    /// FIXME: Limited at 32 because serde... see other comment somewhere about this
    pub subscribers: [Option<SubscriberInfo>; 32],
}
