use crate::{
    status::NotificationKind,
    str::{String32, String8},
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct SubscriberInfo {
    pub identifier: String32,
    pub address: IpAddress,
    pub message_kinds: NotificationKind,
    pub last_contact: u128,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Default, Eq)]
pub struct IpAddress {
    port: u16,
    addr: [u8; 4],
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AudioDevice {
    pub id: String32,
    pub name: String32,
    pub io_size: (usize, usize),
}

impl AudioDevice {
    //pub fn from_aplay_str(str: String32) -> Option<AudioDevice> {
    //    //card 1: Headphones [bcm2835 Headphones], device 0: bcm2835 Headphones [bcm2835 Headphones]
    //    let card_idx = &str[str.find("card")? + 5..str.find(':')?];
    //    let _device_idx = &str[(str.find("device")? + 7)..(str[8..].find(':')? + 8)];
    //    let id = format!("hw:{card_idx}");
    //    Some(Self {
    //        id,
    //        io_size: (0, 0),
    //        name: str,
    //    })
    //}
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct JACKStatus {
    pub available_devices: [Option<AudioDevice>; 8],
    pub io_size: (usize, usize),
    pub buffer_size: usize,
    pub sample_rate: usize,
    pub frame_size: usize,
    pub connections: [u32; 32],
    pub client_name: String32,
    pub output_name: String32,
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct NetworkStatus {
    // FIXME: Limited at 32 because serde... see other comment somewhere about this
    pub subscribers: [Option<SubscriberInfo>; 32],
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

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Heartbeat {
    pub common_version: String8,
    pub system_version: String8,
    pub system_time: u64,
    pub cpu_use_audio: f32,
    pub process_freq_main: usize,
}
