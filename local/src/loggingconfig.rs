use core::fmt;

bitflags::bitflags! {
    /// Bitflag for different types of logs
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Default, Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Copy)]
    pub struct LogKind: u8 {
        /// Error, something has gone wrong
        const Error = 0x01;
        /// Warning, something might be going wrong, or could be going wrong soon
        const Warning = 0x02;
        /// Note, information to the user
        const Note = 0x04;
        /// Command, information that a request from a client has been executed
        const Command = 0x08;
        /// Debugging, generally cluttersome
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
    /// Bitflag for various contexts from which log messages can source
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Default, Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Copy)]
    pub struct LogContext: u8 {
        /// The logging device itself
        const Logger = 0x01;
        /// Network handling and client-server communication
        const Network = 0x02;
        /// Audio processor and the audio thread
        const AudioProcessor = 0x04;
        /// A specific channel's audio source
        const AudioSource = 0x08;
        /// Audio handler, audio management on the main thread
        const AudioHandler = 0x10;
        /// Boot - startup, updates and file management
        const Boot = 0x20;
    }
}

impl LogContext {
    /// Get a human readable name of this LogContext
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

/// Wrapper configuration for the logging device
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct LoggerConfiguration {
    /// Which log-kinds to log. Ignores all others.
    pub active_kinds: LogKind,
    /// Which log-contexts to log. Ignores all others.
    pub active_contexts: LogContext,
}
