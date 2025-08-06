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
        let file = match std::fs::read(path) {
            Ok(val) => val,
            Err(err) => return Err(serde_json::Error::custom("could not read file")),
        };

        let file_str = match std::str::from_utf8(&file) {
            Ok(val) => val,
            Err(err) => return Err(serde_json::Error::custom("invalid utf8")),
        };
        serde_json::from_str::<Show>(file_str)
    }
}
