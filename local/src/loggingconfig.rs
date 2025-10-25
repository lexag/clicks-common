use core::fmt;
use serde::{Deserialize, Serialize};

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
