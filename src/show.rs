use crate::cue::Cue;
use serde::{de::Error, Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShowMetadata {
    pub name: String,
    pub date: String,
    pub credits: Vec<ShowCreditGroup>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShowCreditGroup {
    name: String,
    members: Vec<ShowCredit>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShowCredit {
    name: String,
    role: String,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
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

    pub fn from_file(path: PathBuf) -> Result<Show, serde_json::Error> {
        serde_json::from_str::<Show>(std::str::from_utf8(&std::fs::read(path).unwrap()).unwrap())
    }
}
