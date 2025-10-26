use core::fmt::{Display, Formatter, Result};
use serde::{Deserialize, Serialize};

/// A SMPTE LTC timestamp, including frame rate.
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TimecodeInstant {
    /// Frame rate in frames per second
    pub frame_rate: usize,
    /// Current number of hours
    pub h: i16,
    /// Current number of minutes
    pub m: i16,
    /// Current number of seconds
    pub s: i16,
    /// Current number of frames
    pub f: i16,
    /// Current progress through the current frame, 0-65536
    pub frame_progress: u16,
}

impl PartialEq for TimecodeInstant {
    fn eq(&self, other: &TimecodeInstant) -> bool {
        self.f == other.f && self.s == other.s && self.m == other.m && self.h == other.h
    }
}

impl Display for TimecodeInstant {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}:{:0>2}",
            self.h, self.m, self.s, self.f
        )
    }
}

impl TimecodeInstant {
    /// Create a 00:00:00:00 timecode instant with the given frame rate
    pub fn new(frame_rate: usize) -> TimecodeInstant {
        TimecodeInstant {
            frame_rate,
            ..Default::default()
        }
    }

    /// Add an amount of frame progress to the current timestamp.
    /// If this reaches the end of the frame, f increments and the remaining progress adds to the
    /// next frame.
    pub fn add_progress(&mut self, progress: u16) {
        let prog_of = self.frame_progress as u32 + progress as u32;
        self.frame_progress = (prog_of % 65536) as u16;
        if prog_of >= 65536 {
            self.f += 1
        }
        self.propagate();
    }

    /// Add an amount of microseconds to this timestamp.
    pub fn add_us(&mut self, time_us: u64) {
        self.f += (time_us * self.frame_rate as u64 / 1000000) as i16;
        self.propagate();
    }
    /// Subtract an amount of microseconds from this timestamp.
    pub fn sub_us(&mut self, time_us: u64) {
        let mut tci = TimecodeInstant::new(self.frame_rate);
        tci.add_us(time_us);
        self.sub(tci);
    }

    /// Subtract another [TimecodeInstant] from this timestamp.
    pub fn sub(&mut self, other: TimecodeInstant) {
        self.f -= other.f;
        self.s -= other.s;
        self.m -= other.m;
        self.h -= other.h;
        self.propagate();
    }

    /// Set the current timestamp
    pub fn set_time(&mut self, h: usize, m: usize, s: usize, f: usize) {
        self.h = h as i16;
        self.m = m as i16;
        self.s = s as i16;
        self.f = f as i16;
    }

    // propagate changes to f into the other values
    fn propagate(&mut self) {
        self.s += self.f / self.frame_rate as i16;
        self.f %= self.frame_rate as i16;
        self.f += self.frame_rate as i16;
        self.f %= self.frame_rate as i16;
        self.m += self.s / 60;
        self.s %= 60;
        self.h += self.m / 60;
        self.m %= 60;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_progress() {
        let time_const = TimecodeInstant::new(25);
        let mut time = time_const.clone();
        time.add_progress(0);
        assert_eq!(time, time_const);
        time.add_progress(u16::MAX);
        time.add_progress(1);
        assert_eq!(time.frame_progress, 0);
        assert_eq!(time.f, 1);
        time.add_progress(1);
        assert_eq!(time.frame_progress, 1);
    }
    #[test]
    fn add_sub_identity() {
        let time_const = TimecodeInstant::new(25);
        for i in (0..36000 * 1000000).step_by(123456) {
            println!("Adding and subtracting {i}");
            let mut time = time_const.clone();
            time.add_us(i);
            time.sub_us(i);
            assert_eq!(time, time_const, "Failed with {}us ({} s)", i, i / 1000000);
        }
    }
}
