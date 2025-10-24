use crate::cue::Cue;
use crate::event::BeatEventContainer;

// TODO: unit tests :))
pub struct EventCursor<'a> {
    cursor: u8,
    cue: &'a Cue,
}

impl<'a> EventCursor<'a> {
    pub fn new(cue: &'a Cue) -> EventCursor<'a> {
        Self { cursor: 0, cue }
    }

    pub fn seek(&mut self, location: u16) {
        if self.location() > location {
            self.cursor = 0;
        }
        while self.location() < location && self.cursor < Cue::EVENT_SLOTS as u8 {
            self.cursor += 1;
        }
    }

    pub fn location(&self) -> u16 {
        self.cue.events[self.cursor as usize].location
    }

    pub fn get(&mut self) -> BeatEventContainer {
        self.cue.events[self.cursor as usize]
    }

    pub fn step(&mut self) {
        self.cursor += (self.cursor + 1).min(Cue::EVENT_SLOTS as u8);
    }
}
