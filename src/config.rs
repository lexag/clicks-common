use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum BootProgramOrder {
    #[default]
    Run,
    WriteConfig,
    Upgrade,
    ExtractLogs,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SystemConfiguration {
    pub audio: AudioConfiguration,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfiguration {
    pub server: JACKServerConfiguration,
    pub client: JACKClientConfiguration,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JACKServerConfiguration {
    pub device_name: String,
    pub system_name: String,
    pub sample_rate: usize,
    pub num_channels: usize,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JACKClientConfiguration {
    pub name: String,
}
