use mem::str::String32;
use serde::{Deserialize, Serialize};

/// Wrapper type combining both JACK client and JACK server configurations
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(default)]
pub struct AudioConfiguration {
    /// Server configuration
    pub server: JACKServerConfiguration,
    /// Client configuration
    pub client: JACKClientConfiguration,
}

/// JACK server configuration. The JACK server is the audio thread handler running on the OS of the
/// ClicKS core machine.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(default)]
pub struct JACKServerConfiguration {
    /// Device id for the connected audio device, either external or internal
    pub device_id: String32,
    /// System name, used for port searching
    pub system_name: String32,
    /// Sample rate, in Hz
    pub sample_rate: u16,
    /// Period size, in samples
    pub period_size: u16,
}

/// JACK client configuration. The JACK client is *not* a ClicKS client, rather it is the core's
/// audio client that connects to the audio server. JACK server and client both run on the core
/// machine.
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Copy)]
#[serde(default)]
pub struct JACKClientConfiguration {
    /// Name of this client
    pub name: String32,
}
