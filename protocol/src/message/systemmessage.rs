use crate::message::heartbeat::Heartbeat;
use local::{
    config::SystemConfiguration,
    status::{BeatState, CueState, JACKStatus, NetworkStatus, TransportState},
};
use mem::typeflags::MessageType;
use serde::{Deserialize, Serialize};

/// Definition of messages sent from core to client
#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Message {
    /// Transport state has changed. Sent once whenever jumping or starting/stopping playback, and
    /// multiple times per second during playback.
    TransportChanged(TransportState),
    /// Current beat has changed. Sent at the start of a new beat during playback
    BeatChanged(BeatState),
    /// Current cue has changed. Sent whenever a new cue is loaded.
    CueChanged(CueState),
    /// Network status has changed. Sent when a subscriber joins or leaves the network
    NetworkChanged(NetworkStatus),
    /// JACK state changed, sent on startup to report available audio devices, and then again when
    /// an audio device is selected.
    JACKStateChanged(JACKStatus),
    /// System configuration has changed. Sent (to all subscribers) as a response to a
    /// ChangeSystemConfiguration request. Also sent on startup
    ConfigurationChanged(SystemConfiguration),
    /// A shutdown has been requested. Sent to all subscribers on a Shutdown request, telling them
    /// to unsubscribe and/or disconnect, and expect to not receive subsequent Heartbeats
    ShutdownOccured,
    /// Sent every few seconds to all clients, reporting core status and making sure the connection
    /// is alive
    Heartbeat(Heartbeat),
}

impl Message {
    /// Get the type flag of this message
    pub fn to_type(&self) -> MessageType {
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
