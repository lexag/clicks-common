use core::fmt;
use serde::{Deserialize, Serialize};

/// Beat represent a musical beat, or a subdivision thereof
#[derive(Clone, Serialize, Deserialize, Default, PartialEq, Copy)]
pub struct Beat {
    /// Beat number, first beat in a bar typically has 1, followed by 2 etc. Can be between 0 and 255.
    pub count: u8,
    /// Bar number, first bar in a cue typically has 1, followed by 2 etc. Can be between 0 and
    /// 255.
    pub bar_number: u8,
    /// Length of this beat in microseconds. 0-4_294_967_295 (u32) means tempos between one beat
    /// per microsecond and one beat per approx 1h 11m.
    /// In practice, the top speed of beat processing is limited by core performance, and a beat
    /// processing speed of 1MHz is probably unlikely.
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
    /// Returns an empty Beat
    /// Does not have any length, and is thus unstable for playback.
    /// Should not be used to directly generate played Beats, just to fill out collections with
    /// beats to be edited.
    pub fn empty() -> Beat {
        Beat {
            count: 0,
            bar_number: 0,
            length: 0,
        }
    }

    /// Tempo this beat implies in BPM.
    /// Note that this does not necessary correlate to pulse, as a pulse-beat may be subdivided
    /// into multiple metronome beats.
    pub fn tempo(&self) -> u16 {
        if self.length == 0 {
            return 0;
        }
        (60000000.0 / self.length as f32).round() as u16
    }

    /// Is this beat null (opposite of populated), i.e. is this beat just an empty slot in the beat
    /// table (true), or an actual beat (false)?
    pub fn is_null(&self) -> bool {
        self.length == 0 && self.bar_number == 0 && self.count == 0
    }
}
