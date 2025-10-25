use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MessageType: u8 {
        const TransportChanged = 0x01;
        const BeatChanged = 0x01;
        const CueChanged = 0x02;
        const ShowChanged = 0x04;
        const NetworkChanged = 0x08;
        const JACKStateChanged = 0x10;
        const ConfigurationChanged = 0x20;
        const ShutdownOccured = 0x40;
        const Heartbeat = 0x80;
    }
}
