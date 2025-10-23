use crate::{cue::Cue, str::String32};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShowMetadata {
    pub name: String32,
    pub date: String32,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Show {
    pub metadata: ShowMetadata,
    pub cues: [Cue; 32], //FIXME: 32 cues because that is the maximum serde allows. BigArray
                         //exists, but it's very fiddly to nest them and we definitely need it one
                         //layer down for the 512 beats per cue.
                         //
                         //Also, not serde is maybe an alternative. JSON do be kinda ugly
}

impl Show {
    // TODO: lightweight show and cue objects should be separate objects, since const-size
    // revolution
    //pub fn lightweight(&self) -> Self {
    //    Self {
    //        metadata: self.metadata.clone(),
    //        cues: self
    //            .cues
    //            .iter()
    //            .map(|c| Cue {
    //                metadata: c.metadata.clone(),
    //                beats: ,
    //            })
    //            .collect(),
    //    }
    //}

    // TODO: from_file belongs in ShowBuilder, which does not exist yet
    //pub fn from_file(path: PathBuf) -> Result<Show, serde_json::Error> {
    //    let file = match std::fs::read(path) {
    //        Ok(val) => val,
    //        Err(err) => return Err(serde_json::Error::custom("could not read file")),
    //    };

    //    let file_str = match std::str::from_utf8(&file) {
    //        Ok(val) => val,
    //        Err(err) => return Err(serde_json::Error::custom("invalid utf8")),
    //    };
    //    serde_json::from_str::<Show>(file_str)
    //}
}
