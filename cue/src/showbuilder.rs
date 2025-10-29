#![allow(unused_imports)]
use crate::show::Show;
use serde::de::Error;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
#[allow(dead_code)]
/// Builder type for [Show]
pub struct ShowBuilder {}

#[cfg(feature = "std")]
#[allow(dead_code)]
impl ShowBuilder {
    /// Load a [Show] from a show.json file
    /// Returns an error if the file can't be read or if the json is invalid utf8
    pub fn from_file(path: std::path::PathBuf) -> Result<Show, serde_json::Error> {
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
