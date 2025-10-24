use crate::show::Show;
use serde::de::Error;
use std::path::PathBuf;

pub struct ShowBuilder {}

impl ShowBuilder {
    pub fn from_file(path: PathBuf) -> Result<Show, serde_json::Error> {
        let file = match std::fs::read(path) {
            Ok(val) => val,
            Err(_err) => return Err(serde_json::Error::custom("could not read file")),
        };

        let file_str = match std::str::from_utf8(&file) {
            Ok(val) => val,
            Err(_err) => return Err(serde_json::Error::custom("invalid utf8")),
        };
        serde_json::from_str::<Show>(file_str)
    }
}
