use crate::request::command::ControlAction;
use serde::{Deserialize, Serialize};
use status::{
    config::SystemConfigurationChange,
    network::{ConnectionInfo, SubscriberInfo},
};

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
