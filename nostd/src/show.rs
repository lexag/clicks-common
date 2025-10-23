use crate::{
    cue::{Cue, CueSkeleton},
    str::String32,
};
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShowSkeleton {
    pub metadata: ShowMetadata,
    pub cues: [CueSkeleton; 32],
}
