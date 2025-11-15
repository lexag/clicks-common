#![allow(unused_imports)]
#[cfg(feature = "std")]
use crate::show::Show;

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
    pub fn from_file(
        path: std::path::PathBuf,
    ) -> Result<(Show, usize), bincode::error::DecodeError> {
        use bincode::config::{BigEndian, Config, Configuration, Fixint};

        let file = match std::fs::read(path) {
            Ok(val) => val,
            Err(_err) => return Err(bincode::error::DecodeError::Other("file read error")),
        };

        let config = Configuration::default();
        bincode::decode_from_slice::<Show, Configuration<BigEndian, Fixint>>(&file, config)
    }
}
