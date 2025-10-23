use crate::str::String32;
use core::fmt;
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

bitflags::bitflags! {
    #[derive(Default, Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
    pub struct LogKind: u8 {
        const Error = 0x01;
        const Warning = 0x02;
        const Note = 0x04;
        const Command = 0x08;
        const Debug = 0x10;
    }
}

impl fmt::Display for LogKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            LogKind::Error => write!(f, "ERROR"),
            LogKind::Warning => write!(f, "WARNING"),
            LogKind::Note => write!(f, "NOTE"),
            LogKind::Command => write!(f, "COMMAND"),
            LogKind::Debug => write!(f, "DEBUG"),
            _ => write!(f, "multiple flags"),
        }
    }
}

bitflags::bitflags! {
    #[derive(Default, Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
    pub struct LogContext: u8 {
        const Logger = 0x01;
        const Network = 0x02;
        const AudioProcessor = 0x04;
        const AudioSource = 0x08;
        const AudioHandler = 0x10;
        const Boot = 0x20;
    }
}

impl LogContext {
    pub fn get_name(&self) -> &str {
        match *self {
            LogContext::Logger => "Logger",
            LogContext::Network => "Network",
            LogContext::AudioProcessor => "AudioProcessor",
            LogContext::AudioSource => "AudioSource",
            LogContext::AudioHandler => "AudioHandler",
            LogContext::Boot => "Boot",
            _ => "multiple flags",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(default)]
pub struct LoggerConfiguration {
    pub active_kinds: LogKind,
    pub active_contexts: LogContext,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
pub enum ChannelAssignment {
    L,
    R,
    #[default]
    Mono,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(default)]
pub struct ChannelConfiguration {
    pub name: String32,
    pub channel_assignment: ChannelAssignment,
    pub gain: f32,
}
