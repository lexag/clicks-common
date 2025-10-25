use crate::{
    audioconfig::AudioConfiguration, channelconfig::ChannelConfiguration,
    loggingconfig::LoggerConfiguration,
};
use mem::str::String32;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum BootProgramOrder {
    #[default]
    Run,
    WriteConfig,
    Upgrade,
    ExtractLogs,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct SystemConfiguration {
    pub audio: AudioConfiguration,
    pub logger: LoggerConfiguration,
    pub channels: [ChannelConfiguration; 32],
}

impl Default for SystemConfiguration {
    fn default() -> Self {
        let mut a = Self {
            audio: AudioConfiguration::default(),
            logger: LoggerConfiguration::default(),
            channels: [ChannelConfiguration {
                name: String32::empty(),
                ..ChannelConfiguration::default()
            }; 32],
        };
        a.channels[0].name = String32::new("Metronome");
        a.channels[0].name = String32::new("Timecode");
        for i in 2..32 {
            a.channels[i].name = String32::new("i");
        }
        a
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SystemConfigurationChange {
    ChangeAudioConfiguration(AudioConfiguration),
    ChangeLoggerConfiguration(LoggerConfiguration),
    ChangeChannelConfiguration(u8, ChannelConfiguration),
}
