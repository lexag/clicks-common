use crate::cue::Cue;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ShowMetadata {
    pub name: String,
    pub date: String,
    pub credits: Vec<ShowCreditGroup>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ShowCreditGroup {
    name: String,
    members: Vec<ShowCredit>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ShowCredit {
    name: String,
    role: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct Show {
    pub metadata: ShowMetadata,
    pub cues: Vec<Cue>,
}

impl Show {
    pub fn lightweight(&self) -> Self {
        Self {
            metadata: self.metadata.clone(),
            cues: self
                .cues
                .iter()
                .map(|c| Cue {
                    metadata: c.metadata.clone(),
                    beats: vec![],
                })
                .collect(),
        }
    }
}
