use crate::event::Event;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct EventTable {
    #[serde(with = "BigArray")]
    table: [Event; Self::SIZE],
}

impl EventTable {
    pub const SIZE: usize = 64;

    pub fn empty() -> Self {
        Self {
            table: [Event::default(); Self::SIZE],
        }
    }

    pub fn get(&self, idx: u8) -> Event {
        if idx > Self::SIZE as u8 {
            Event::default()
        } else {
            self.table[idx as usize]
        }
    }

    pub fn set(&mut self, idx: u8, event: Event) -> bool {
        if idx > Self::SIZE as u8 {
            false
        } else {
            self.table[idx as usize] = event;
            true
        }
    }

    pub fn sort(&mut self) {
        self.table.sort_by_key(|o| o.location);
    }
}
