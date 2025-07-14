use serde::{Deserialize, Serialize};
use std::{collections::HashMap, default, fmt::Display};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum BootProgramOrder {
    #[default]
    Run,
    WriteConfig,
    Upgrade,
    ExtractLogs,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct SystemConfiguration {
    pub audio: AudioConfiguration,
    pub logger: LoggerConfiguration,
    pub channels: ChannelsConfiguration,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ChannelsConfiguration {
    pub channels: Vec<ChannelConfiguration>,
}

impl Default for ChannelsConfiguration {
    fn default() -> Self {
        Self {
            channels: vec![
                ChannelConfiguration {
                    name: "Metronome".to_string(),
                    ..ChannelConfiguration::default()
                },
                ChannelConfiguration {
                    name: "Timecode".to_string(),
                    ..ChannelConfiguration::default()
                },
            ]
            .into_iter()
            .chain(
                (0..30)
                    .map(|i| ChannelConfiguration::new(i))
                    .collect::<Vec<ChannelConfiguration>>(),
            )
            .collect(),
        }
    }
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AudioConfiguration {
    pub server: JACKServerConfiguration,
    pub client: JACKClientConfiguration,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JACKServerConfiguration {
    pub device_id: String,
    pub system_name: String,
    pub sample_rate: usize,
    pub num_channels: usize,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct JACKClientConfiguration {
    pub name: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum LogKind {
    Error,
    Warning,
    Note,
    Command,
    #[default]
    Debug,
}

impl LogKind {
    pub fn get_name(&self) -> &str {
        match self {
            LogKind::Error => "Error",
            LogKind::Warning => "Warning",
            LogKind::Note => "Note",
            LogKind::Command => "Command",
            LogKind::Debug => "Debug",
        }
    }
}

impl Display for LogKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LogKind::Error => write!(f, "ERROR"),
            LogKind::Warning => write!(f, "WARNING"),
            LogKind::Note => write!(f, "NOTE"),
            LogKind::Command => write!(f, "COMMAND"),
            LogKind::Debug => write!(f, "DEBUG"),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum LogContext {
    #[default]
    Logger,
    Network,
    AudioProcessor,
    AudioSource,
    AudioHandler,
    Boot,
}

impl LogContext {
    pub fn get_name(&self) -> &str {
        match self {
            LogContext::Logger => "Logger",
            LogContext::Network => "Network",
            LogContext::AudioProcessor => "AudioProcessor",
            LogContext::AudioSource => "AudioSource",
            LogContext::AudioHandler => "AudioHandler",
            LogContext::Boot => "Boot",
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggerConfiguration {
    pub toggle_kinds: HashMap<LogKind, bool>,
    pub toggle_contexts: HashMap<LogContext, bool>,
}

impl Default for LoggerConfiguration {
    fn default() -> Self {
        Self {
            toggle_contexts: HashMap::from([
                (LogContext::Logger, false),
                (LogContext::Network, false),
                (LogContext::AudioProcessor, false),
                (LogContext::AudioSource, false),
                (LogContext::AudioHandler, false),
                (LogContext::Boot, false),
            ]),
            toggle_kinds: HashMap::from([
                (LogKind::Error, false),
                (LogKind::Warning, false),
                (LogKind::Note, false),
                (LogKind::Command, false),
                (LogKind::Debug, false),
            ]),
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChannelAssignment {
    L,
    R,
    #[default]
    Mono,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ChannelConfiguration {
    pub name: String,
    pub description: String,
    pub channel_assignment: ChannelAssignment,
    pub gain: f32,
}

impl ChannelConfiguration {
    pub fn new(idx: usize) -> Self {
        Self {
            name: format!("Ch. {idx}"),
            description: "".to_string(),
            channel_assignment: ChannelAssignment::Mono,
            gain: 0.0f32,
        }
    }
}
