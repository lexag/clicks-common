use crate::event::Event;

/// Table of events that occur in a specific cue.
#[derive(Clone, Debug, PartialEq, Copy, bincode::Encode, bincode::Decode)]
pub struct EventTable {
    table: [Event; Self::SIZE],
    table_ptr: u8,
}

impl EventTable {
    /// EventTable is const-size to support uC and low level network communication, and must
    /// therefore have a constant size. 64 events per cue should be plenty for most situtations.
    pub const SIZE: usize = 32;

    /// Create an empty EventTable.
    pub const fn empty() -> Self {
        Self {
            table: [Event::null(); Self::SIZE],
            table_ptr: 0,
        }
    }

    /// Get the event at idx, returning a null-event if idx is out of bounds.
    pub fn get(&self, idx: u8) -> Option<Event> {
        let e = self.table.get(idx as usize)?;
        if e.is_null() {
            None
        } else {
            Some(*e)
        }
    }

    /// Get a mut ref to the event at idx, returning a null-event if idx is out of bounds.
    pub fn get_mut(&mut self, idx: u8) -> Option<&mut Event> {
        let e = self.table.get_mut(idx as usize)?;
        if e.is_null() {
            None
        } else {
            Some(e)
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
        tiny_sort::stable::sort_by_key(&mut self.table, |e: &Event| e.location);
    }

    /// Add an event to this table.
    /// The table will be sorted to place the new event in order.
    /// This may break existing [crate::eventcursor::EventCursor]s
    pub fn push(&mut self, event: Event) -> bool {
        if self.table_ptr as usize >= Self::SIZE - 1 {
            return false;
        }
        self.set(self.table_ptr, event);
        self.table_ptr += 1;
        self.sort();
        true
    }

    /// Remove an event from the table by index.
    /// The event is returned and the table is resorted to fill the gap
    pub fn pop(&mut self, idx: u8) -> Option<Event> {
        let e = self.get(idx)?;
        self.set(idx, Event::null());
        self.sort();
        Some(e)
    }
}
