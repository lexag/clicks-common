use crate::cue::Cue;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};

#[derive(Serialize, Deserialize)]
struct ShowMetadata {
    name: String,
    date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Show {
    metadata: ShowMetadata,
    pub cues: Vec<Cue>,
}

impl Show {
    pub fn new() -> Show {
        Show {
            metadata: ShowMetadata {
                name: "some name".to_string(),
                date: "some date".to_string(),
            },
            cues: vec![],
        }
    }

    pub fn from_file(path: &str) -> Result<Show, ()> {
        if let Ok(json_str) = read_to_string(path) {
            let parser: Show =
                serde_json::from_str(&json_str.to_string()).expect("bad json parsing");
            return Ok(parser);
        } else {
            println!("Showfile {path} not found");
            return Err(());
        }
    }

    pub fn to_file(&self, path: &str) {
        let _ = write(path, serde_json::to_string_pretty(self).unwrap());
    }
}
