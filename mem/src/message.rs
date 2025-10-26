use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    /// Bitflag for one or multiple types of core -> client message
    #[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MessageType: u8 {
        /// Transport changed
        const TransportChanged = 0x01;
        /// Beat changed
        const BeatChanged = 0x01;
        /// Cue changed
        const CueChanged = 0x02;
        /// Show changed
        const ShowChanged = 0x04;
        /// Network changed
        const NetworkChanged = 0x08;
        /// JACKState changed
        const JACKStateChanged = 0x10;
        /// Configuration changed
        const ConfigurationChanged = 0x20;
        /// Shutdown occured
        const ShutdownOccured = 0x40;
        /// Heartbeat
        const Heartbeat = 0x80;
    }
}
