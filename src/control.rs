use crate::{
    command::ControlCommand,
    config::SystemConfiguration,
    network::{ConnectionInfo, SubscriberInfo},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControlMessageKind {
    NotifySubscribers,
    Shutdown,
    Initialize,
    RoutingChangeRequest,
    ControlCommand,
    SubscribeRequest,
    UnsubscribeRequest,
    SetConfigurationRequest,
    Ping,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ControlMessage {
    NotifySubscribers,
    Shutdown,
    Initialize,
    RoutingChangeRequest(usize, usize, bool),
    ControlCommand(ControlCommand),
    SubscribeRequest(SubscriberInfo),
    UnsubscribeRequest(ConnectionInfo),
    SetConfigurationRequest(SystemConfiguration),
    Ping,
}

impl ControlMessage {
    pub fn to_kind(&self) -> ControlMessageKind {
        match self {
            Self::NotifySubscribers => ControlMessageKind::NotifySubscribers,
            Self::Shutdown => ControlMessageKind::Shutdown,
            Self::Initialize => ControlMessageKind::Initialize,
            Self::RoutingChangeRequest(..) => ControlMessageKind::RoutingChangeRequest,
            Self::ControlCommand(..) => ControlMessageKind::ControlCommand,
            Self::SubscribeRequest(..) => ControlMessageKind::SubscribeRequest,
            Self::UnsubscribeRequest(..) => ControlMessageKind::UnsubscribeRequest,
            Self::SetConfigurationRequest(..) => ControlMessageKind::SetConfigurationRequest,
            Self::Ping => ControlMessageKind::Ping,
        }
    }
}
