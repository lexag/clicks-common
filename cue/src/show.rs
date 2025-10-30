use crate::cue::{Cue, CueSkeleton};
use mem::str::String32;
use serde::{Deserialize, Serialize};

/// A Show represents a collection of Cues for semi-linear sequential playback
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub struct Show {
    /// Metadata for this show
    pub metadata: ShowMetadata,
    /// Cue table for this show.
    ///
    /// FIXME: 32 cues because that is the maximum serde allows. BigArray
    /// exists, but it's very fiddly to nest them and we definitely need it one
    /// layer down for the 512 beats per cue.
    pub cues: [Cue; 32],
}

/// Metadata for a Show instance. Like with [crate::cue::CueMetadata], anything that is human readable and
/// might be of interest to anyone without in-depth technical knowledge about the inner workings
/// of ClicKS should be in ShowMetadata in a human readable format.
#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub struct ShowMetadata {
    /// Name of this show. Usually the name of the production
    pub name: String32,
    /// User-defined date field. Can be used for date of show programming or date of show
    /// performance.
    pub date: String32,
}

/// Lightweight shadow of [Show] for network and uC purposes, see [CueSkeleton]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
pub struct ShowSkeleton {
    /// Metadata for this show
    pub metadata: ShowMetadata,
    /// Cue table for this show.
    pub cues: [CueSkeleton; 32],
}

impl ShowSkeleton {
    /// Create a new ShowSkeleton from a full show
    pub fn new(show: Show) -> Self {
        Self {
            metadata: show.metadata,
            cues: show.cues.map(CueSkeleton::new),
        }
    }

    /// Create a full show from this skeleton
    pub fn to_show(self) -> Show {
        Show {
            metadata: self.metadata,
            cues: self.cues.map(|c| c.to_cue()),
        }
    }
}
