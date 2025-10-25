use crate::{message::MessageType, str::String32};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct SubscriberInfo {
    pub identifier: String32,
    pub address: IpAddress,
    pub message_kinds: MessageType,
    pub last_contact: u128,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct IpAddress {
    port: u16,
    addr: [u8; 4],
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ConnectionEnd {
    Server,
    Client,
    Local,
    Remote,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ConnectionInfo {
    pub identifier: String32,
    pub end: ConnectionEnd,
    pub address: IpAddress,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        ConnectionInfo {
            end: ConnectionEnd::Local,
            identifier: String32::new("Unknown identifier"),
            address: IpAddress::default(),
        }
    }
}
