use crate::show::Show;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum BootProgramOrder {
    #[default]
    Run,
    WriteConfig,
    Upgrade,
    ExtractLogs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootConfig {
    pub boot_order: BootProgramOrder,
    pub audio: Option<AudioConfiguration>,
    pub show: Option<Show>,
}

impl Default for BootConfig {
    fn default() -> Self {
        Self {
            boot_order: BootProgramOrder::Run,
            audio: Some(AudioConfiguration::default()),
            show: Some(Show::default()),
        }
    }
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
