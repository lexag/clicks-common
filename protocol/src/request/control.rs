use crate::request::command::ControlAction;
use local::config::SystemConfigurationChange;
use mem::{
    network::{ConnectionInfo, SubscriberInfo},
    typeflags::RequestType,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Request {
    NotifySubscribers,
    Shutdown,
    Initialize,
    ChangeRouting(usize, usize, bool),
    ControlAction(ControlAction),
    Subscribe(SubscriberInfo),
    Unsubscribe(ConnectionInfo),
    ChangeConfiguration(SystemConfigurationChange),
    Ping,
}

impl Request {
    pub fn to_kind(&self) -> RequestType {
        match self {
            Self::NotifySubscribers => RequestType::NotifySubscribers,
            Self::Shutdown => RequestType::Shutdown,
            Self::Initialize => RequestType::Initialize,
            Self::ChangeRouting(..) => RequestType::RoutingChange,
            Self::ControlAction(..) => RequestType::ControlCommand,
            Self::Subscribe(..) => RequestType::Subscribe,
            Self::Unsubscribe(..) => RequestType::Unsubscribe,
            Self::ChangeConfiguration(..) => RequestType::SetConfiguration,
            Self::Ping => RequestType::Ping,
        }
    }
}
