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
    //    pub fn from_file(path: &str) -> Result<Show, ()> {
    //        if let Ok(json_str) = read_to_string(path) {
    //            let parser: Show =
    //                serde_json::from_str(&json_str.to_string()).expect("bad json parsing");
    //            return Ok(parser);
    //        } else {
    //            println!("Showfile {path} not found");
    //            return Err(());
    //        }
    //    }
    //
    //    pub fn to_file(&self, path: &str) {
    //        let _ = write(path, serde_json::to_string_pretty(self).unwrap());
    //    }
}
