use std::{fmt::Debug, net::Ipv4Addr, str::FromStr};

use crate::status::NotificationKind;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubscriberInfo {
    pub identifier: String,
    pub address: String,
    pub port: String,
    pub message_kinds: Vec<NotificationKind>,
    pub last_contact: String,
}

impl SubscriberInfo {
    pub fn get_ipv4_object(&self) -> Ipv4Addr {
        Ipv4Addr::from_str(&format!("{}:{}", &self.address, &self.port).to_string()).unwrap()
    }

    // lol function name
    pub fn strstreq(&self, address: String, port: String) -> bool {
        self.address == address && self.port == port
    }

    pub fn streq(&self, ipv4: String) -> bool {
        format!("{}:{}", self.address, self.port) == ipv4
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub io_size: (usize, usize),
}

impl AudioDevice {
    pub fn from_aplay_str(str: String) -> Option<AudioDevice> {
        //card 1: Headphones [bcm2835 Headphones], device 0: bcm2835 Headphones [bcm2835 Headphones]
        let card_idx = &str[str.find("card")? + 5..str.find(':')?];
        let _device_idx = &str[(str.find("device")? + 7)..(str[8..].find(':')? + 8)];
        let id = format!("hw:{card_idx}");
        Some(Self {
            id,
            io_size: (0, 0),
            name: str,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct JACKStatus {
    pub available_devices: Vec<AudioDevice>,
    pub io_size: (usize, usize),
    pub buffer_size: usize,
    pub sample_rate: usize,
    pub frame_size: usize,
    pub connections: Vec<(usize, usize)>,
    pub client_name: String,
    pub output_name: String,
    pub running: bool,
}

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct NetworkStatus {
    pub subscribers: Vec<SubscriberInfo>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum ConnectionEnd {
    Server,
    Client,
    Local,
    Remote,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ConnectionInfo {
    pub identifier: String,
    pub end: ConnectionEnd,
    pub address: String,
    pub port: String,
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        ConnectionInfo {
            end: ConnectionEnd::Local,
            identifier: String::new(),
            address: "127.0.0.1".to_owned(),
            port: "0".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Heartbeat {
    pub system_time: u64,
    pub cpu_use_audio: f32,
    pub process_freq_main: usize,
}
