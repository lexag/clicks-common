use crate::{message::MessageType, str::StaticString};
use core::fmt;

/// Information about a client subscribing to core messages
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Default, Eq, Copy, bincode::Encode, bincode::Decode)]
pub struct SubscriberInfo {
    /// Human readable identifier, such as device name or user
    pub identifier: StaticString<32>,
    /// Ipv4 address of this device
    pub address: IpAddress,
    /// What message types does this client want to receive?
    pub message_kinds: MessageType,
    /// Last contact in unix timestamp
    pub last_contact: u128,
}

/// Ipv4 address and udp port information
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Default, Eq, Copy, bincode::Encode, bincode::Decode)]
pub struct IpAddress {
    /// UDP/TCP port number 0-65535
    pub port: u16,
    /// Ipv4 address octets [192, 168, 1, 150]
    pub addr: [u8; 4],
}

impl IpAddress {
    /// Create a new IpAddress from four address octets and a port number
    pub fn new(address: [u8; 4], port: u16) -> Self {
        Self {
            port,
            addr: address,
        }
    }

    /// Create a new IpAddress from a typical ip string format "192.168.1.150:3030"
    pub fn from_address_str(str: &str) -> Option<Self> {
        let (addr, port) = str.split_once(':')?;
        Some(Self {
            port: port.parse().ok()?,
            addr: IpAddress::octets_from_str(addr)?,
        })
    }

    /// Create a new IpAddress from a typical ip string ("192.168.1.150") and a separate port
    /// number     
    pub fn from_str_and_port(str: &str, port: u16) -> Option<Self> {
        Some(Self {
            port,
            addr: IpAddress::octets_from_str(str)?,
        })
    }

    fn octets_from_str(mut str: &str) -> Option<[u8; 4]> {
        let mut octets: [u8; 4] = [0; 4];
        for i in &mut octets {
            let (octet, rest) = str.split_once('.').or(Some((str, "")))?;
            *i = octet.parse().ok()?;
            str = rest;
        }
        Some(octets)
    }
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}.{}:{}",
            self.addr[0], self.addr[1], self.addr[2], self.addr[3], self.port
        )
    }
}

/// Which end of a network connection is this?
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Copy, bincode::Encode, bincode::Decode)]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Copy, bincode::Encode, bincode::Decode)]
pub struct ConnectionInfo {
    /// Identifier
    pub identifier: StaticString<32>,
    /// Which end of a connection is this?
    pub end: ConnectionEnd,
    /// Ipv4 address
    pub address: IpAddress,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        ConnectionInfo {
            end: ConnectionEnd::Local,
            identifier: StaticString::new("Unknown identifier"),
            address: IpAddress::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ip_from_str() {
        assert_eq!(
            IpAddress::from_address_str("192.168.1.123:12345"),
            Some(IpAddress::new([192, 168, 1, 123], 12345))
        );
        assert_eq!(
            IpAddress::from_address_str("0.0.0.0:0"),
            Some(IpAddress::new([0, 0, 0, 0], 0))
        );
        assert_eq!(IpAddress::from_address_str("300.168.1.123:12345"), None);
        assert_eq!(IpAddress::from_address_str("192.168:1"), None);
    }

    #[test]
    fn octets_from_str() {
        assert_eq!(
            IpAddress::octets_from_str("192.168.1.123"),
            Some([192, 168, 1, 123])
        );
        assert_eq!(IpAddress::octets_from_str("168.1.123"), None);
        assert_eq!(IpAddress::octets_from_str("192.168:1.123"), None);
    }
}
