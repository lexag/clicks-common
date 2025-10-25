use crate::message::heartbeat::Heartbeat;
use local::{
    config::SystemConfiguration,
    status::{BeatState, CueState, JACKStatus, NetworkStatus, TransportState},
};
use mem::typeflags::MessageType;
use serde::{Deserialize, Serialize};

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Message {
    TransportChanged(TransportState),
    BeatChanged(BeatState),
    CueChanged(CueState),
    NetworkChanged(NetworkStatus),
    JACKStateChanged(JACKStatus),
    ConfigurationChanged(SystemConfiguration),
    ShutdownOccured,
    Heartbeat(Heartbeat),
}

impl Message {
    pub fn to_kind(&self) -> MessageType {
        match self {
            Self::TransportChanged(..) => MessageType::TransportChanged,
            Self::BeatChanged(..) => MessageType::BeatChanged,
            Self::CueChanged(..) => MessageType::JACKStateChanged,
            Self::NetworkChanged(..) => MessageType::ShowChanged,
            Self::JACKStateChanged(..) => MessageType::NetworkChanged,
            Self::ConfigurationChanged(..) => MessageType::ConfigurationChanged,
            Self::ShutdownOccured => MessageType::ShutdownOccured,
            Self::Heartbeat(..) => MessageType::Heartbeat,
        }
    }
}
