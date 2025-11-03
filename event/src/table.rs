use crate::event::Event;

/// Table of events that occur in a specific cue.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct EventTable {
    #[cfg_attr(feature = "serde", serde(with = "serde_big_array::BigArray"))]
    table: [Event; Self::SIZE],
    table_ptr: u8,
}

impl EventTable {
    /// EventTable is const-size to support uC and low level network communication, and must
    /// therefore have a constant size. 64 events per cue should be plenty for most situtations.
    pub const SIZE: usize = 64;

    /// Create an empty EventTable.
    pub fn empty() -> Self {
        Self {
            table: [Event::default(); Self::SIZE],
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
