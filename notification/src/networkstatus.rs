use config::network::SubscriberInfo;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct NetworkStatus {
    // FIXME: Limited at 32 because serde... see other comment somewhere about this
    pub subscribers: [Option<SubscriberInfo>; 32],
}
