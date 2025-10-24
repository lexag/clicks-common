use crate::message::heartbeat::Heartbeat;
use mem::message::NotificationKind;
use serde::{Deserialize, Serialize};
use status::{
    beatstate::BeatState, config::SystemConfiguration, cuestate::CueState, jackstatus::JACKStatus,
    networkstatus::NetworkStatus, transportstate::TransportState,
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Notification {
    TransportChanged(TransportState),
    BeatChanged(BeatState),
    CueChanged(CueState),
    NetworkChanged(NetworkStatus),
    JACKStateChanged(JACKStatus),
    ConfigurationChanged(SystemConfiguration),
    ShutdownOccured,
    Heartbeat(Heartbeat),
}

impl Notification {
    pub fn to_kind(&self) -> NotificationKind {
        match self {
            Self::TransportChanged(..) => NotificationKind::TransportChanged,
            Self::BeatChanged(..) => NotificationKind::BeatChanged,
            Self::CueChanged(..) => NotificationKind::JACKStateChanged,
            Self::NetworkChanged(..) => NotificationKind::ShowChanged,
            Self::JACKStateChanged(..) => NotificationKind::NetworkChanged,
            Self::ConfigurationChanged(..) => NotificationKind::ConfigurationChanged,
            Self::ShutdownOccured => NotificationKind::ShutdownOccured,
            Self::Heartbeat(..) => NotificationKind::Heartbeat,
        }
    }
}
