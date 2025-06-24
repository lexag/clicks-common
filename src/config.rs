use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfiguration {
    server: JACKServerConfiguration,
    client: JACKClientConfiguration,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JACKServerConfiguration {}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JACKClientConfiguration {}
