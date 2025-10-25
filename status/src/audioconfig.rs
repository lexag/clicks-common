use mem::str::String32;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct AudioConfiguration {
    pub server: JACKServerConfiguration,
    pub client: JACKClientConfiguration,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JACKServerConfiguration {
    pub device_id: String32,
    pub system_name: String32,
    pub sample_rate: u16,
    pub period_size: u16,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct JACKClientConfiguration {
    pub name: String32,
}
