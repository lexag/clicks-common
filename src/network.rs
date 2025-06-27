use std::{fmt::Debug, net::Ipv4Addr, str::FromStr};

use crate::{command::ControlCommand, cue::Cue, show::Show, status::ProcessStatus};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum StatusMessageKind {
    ProcessStatus(Option<ProcessStatus>),
    CueStatus(Option<Cue>),
    ShowStatus(Option<Show>),
    NetworkStatus(Option<NetworkStatus>),
    JACKStatus(Option<JACKStatus>),
    Shutdown,
}

impl Debug for StatusMessageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            StatusMessageKind::ProcessStatus(None) => write!(f, "ProcessStatus"),
            StatusMessageKind::CueStatus(None) => write!(f, "CueStatus"),
            StatusMessageKind::ShowStatus(None) => write!(f, "ShowStatus"),
            StatusMessageKind::NetworkStatus(None) => write!(f, "NetworkStatus"),
            StatusMessageKind::JACKStatus(None) => write!(f, "JACKStatus"),
            StatusMessageKind::Shutdown => write!(f, "Shutdown"),
            // TODO: This mf.
            _ => write!(f, "<representation todo>"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ControlMessageKind {
    NotifySubscribers,
    Shutdown,
    RoutingChangeRequest(usize, usize, bool),
    ControlCommand(ControlCommand),
    SubscribeRequest(SubscriberInfo),
    UnsubscribeRequest(ConnectionInfo),
    Ping,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SubscriberInfo {
    pub identifier: String,
    pub address: String,
    pub port: String,
    pub message_kinds: Vec<StatusMessageKind>,
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
pub struct JACKStatus {
    pub io_size: (usize, usize),
    pub buffer_size: usize,
    pub sample_rate: usize,
    pub frame_size: usize,
    pub connections: Vec<(usize, usize)>,
    pub client_name: String,
    pub output_name: String,
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
