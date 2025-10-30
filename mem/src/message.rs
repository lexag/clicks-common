use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    /// Bitflag for one or multiple types of core -> client message
    #[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Copy)]
    pub struct MessageType: u16 {
        /// Transport changed
        const TransportData = 0x01;
        /// Beat changed
        const BeatData = 0x02;
        /// Cue changed
        const CueData = 0x04;
        /// Show changed
        const ShowData = 0x08;
        /// Network changed
        const NetworkChanged = 0x10;
        /// JACKState changed
        const JACKStateChanged = 0x20;
        /// Configuration changed
        const ConfigurationChanged = 0x40;
        /// Shutdown occured
        const ShutdownOccured = 0x80;
        /// Heartbeat
        const Heartbeat = 0x100;
    }
}
