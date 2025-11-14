use crate::{
    audioconfig::AudioConfiguration, channelconfig::ChannelConfiguration,
    loggingconfig::LoggerConfiguration,
};
use mem::str::String32;

/// Wrapper configuration type for system configuration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct SystemConfiguration {
    /// Audio configuration values
    pub audio: AudioConfiguration,
    /// Logger configuration values
    pub logger: LoggerConfiguration,
    /// Audio channel configuration values
    pub channels: [ChannelConfiguration; 32],
}

impl SystemConfiguration {
    /// Update self with the provided [SystemConfigurationChange]
    pub fn update(&mut self, change: SystemConfigurationChange) {
        match change {
            SystemConfigurationChange::ChangeAudioConfiguration(config) => self.audio = config,
            SystemConfigurationChange::ChangeLoggerConfiguration(config) => self.logger = config,
            SystemConfigurationChange::ChangeChannelConfiguration(idx, config) => {
                self.channels[idx as usize] = config
            }
        }
    }
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

/// Represents a requested change in a system subconfiguration
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SystemConfigurationChange {
    /// Replace the audio configuration with the provided
    ChangeAudioConfiguration(AudioConfiguration),
    /// Replace the logger configuration with the provided
    ChangeLoggerConfiguration(LoggerConfiguration),
    /// Replace the channel configuration at the provided index with the provided configuration
    ChangeChannelConfiguration(u8, ChannelConfiguration),
}
