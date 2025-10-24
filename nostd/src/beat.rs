use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Copy)]
pub struct Beat {
    pub count: u8,
    pub bar_number: u8,
    pub length: u32,
}

impl fmt::Debug for Beat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Beat")
            .field("count", &self.count)
            .field("length", &self.length)
            .finish()
    }
}

impl Beat {
    pub fn empty() -> Beat {
        Beat {
            count: 0,
            bar_number: 0,
            length: 0,
        }
    }

    //pub fn events_filter<F>(&self, filter: F) -> Vec<BeatEvent>
    //where
    //    F: Fn(&BeatEvent) -> bool,
    //{
    //    self.events.iter().filter(|e| filter(e)).cloned().collect()
    //}

    pub fn tempo(&self) -> u16 {
        if self.length == 0 {
            return 0;
        }
        (60000000.0 / self.length as f32).round() as u16
    }
}
