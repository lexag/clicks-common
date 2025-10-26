use crate::{message::MessageType, str::String32};
use serde::{Deserialize, Serialize};

/// Information about a client subscribing to core messages
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct SubscriberInfo {
    /// Human readable identifier, such as device name or user
    pub identifier: String32,
    /// Ipv4 address of this device
    pub address: IpAddress,
    /// What message types does this client want to receive?
    pub message_kinds: MessageType,
    /// Last contact in unix timestamp
    pub last_contact: u128,
}

/// Ipv4 address and udp port information
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct IpAddress {
    port: u16,
    addr: [u8; 4],
}

/// Which end of a network connection is this?
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub enum ConnectionEnd {
    /// The ClicKS core
    Server,
    /// A ClicKS client
    Client,
    /// Either one, but local to this instance
    Local,
    /// Either one, but not local to this instance
    Remote,
}

/// Information about a connection
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ConnectionInfo {
    /// Identifier
    pub identifier: String32,
    /// Which end of a connection is this?
    pub end: ConnectionEnd,
    /// Ipv4 address
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
