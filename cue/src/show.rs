use crate::cue::{Cue, CueSkeleton};
use mem::str::StaticString;

/// A Show represents a collection of Cues for semi-linear sequential playback
#[derive(Debug, Clone, PartialEq, Copy, bincode::Decode, bincode::Encode)]
pub struct Show {
    /// Metadata for this show
    pub metadata: ShowMetadata,
    /// Cue table for this show.
    pub cues: [Cue; Show::NUM_CUES],
}

impl Show {
    /// Number of slots for cues this type contains.
    pub const NUM_CUES: usize = 64;
}

impl Default for Show {
    fn default() -> Self {
        Self {
            metadata: ShowMetadata::default(),
            cues: [Cue::default(); Show::NUM_CUES],
        }
    }
}

/// Metadata for a Show instance. Like with [crate::cue::CueMetadata], anything that is human readable and
/// might be of interest to anyone without in-depth technical knowledge about the inner workings
/// of ClicKS should be in ShowMetadata in a human readable format.
#[derive(Default, Debug, Clone, PartialEq, Copy, bincode::Encode, bincode::Decode)]
pub struct ShowMetadata {
    /// Name of this show. Usually the name of the production
    pub name: StaticString<32>,
    /// User-defined date field. Can be used for date of show programming or date of show
    /// performance.
    pub date: StaticString<32>,
}

/// Lightweight shadow of [Show] for network and uC purposes, see [CueSkeleton]
#[derive(Debug, Clone, PartialEq, Copy, bincode::Encode, bincode::Decode)]
pub struct ShowSkeleton {
    /// Metadata for this show
    pub metadata: ShowMetadata,
    /// Cue table for this show.
    pub cues: [CueSkeleton; Show::NUM_CUES],
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
