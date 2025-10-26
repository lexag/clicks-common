use crate::event::Event;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

/// Table of events that occur in a specific cue.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EventTable {
    #[serde(with = "BigArray")]
    table: [Event; Self::SIZE],
}

impl EventTable {
    /// EventTable is const-size to support uC and low level network communication, and must
    /// therefore have a constant size. 64 events per cue should be plenty for most situtations.
    pub const SIZE: usize = 64;

    /// Create an empty EventTable.
    pub fn empty() -> Self {
        Self {
            table: [Event::default(); Self::SIZE],
        }
    }

    /// Get the event at idx, returning a null-event if idx is out of bounds.
    pub fn get(&self, idx: u8) -> Event {
        if idx > Self::SIZE as u8 {
            Event::default()
        } else {
            self.table[idx as usize]
        }
    }

    /// Set the event value at idx.
    /// Returns boolean success.
    pub fn set(&mut self, idx: u8, event: Event) -> bool {
        if idx > Self::SIZE as u8 {
            false
        } else {
            self.table[idx as usize] = event;
            true
        }
    }

    /// Sort this table in order of event location.
    /// Unpopulated events (null-events) will be at the end of the table
    pub fn sort(&mut self) {
        self.table.sort_by_key(|o| o.location);
    }
}
