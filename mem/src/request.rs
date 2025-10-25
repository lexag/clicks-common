use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RequestType {
    NotifySubscribers,
    Shutdown,
    Initialize,
    RoutingChange,
    ControlCommand,
    Subscribe,
    Unsubscribe,
    SetConfiguration,
    Ping,
}
