use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    cue::Cue,
    network::{JACKStatus, NetworkStatus},
    show::Show,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlaybackStatus {
    pub clip_idx: usize,
    pub current_sample: i32,
    pub playing: bool,
    pub clips: Vec<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BeatStatus {
    pub beat_idx: usize,
    pub next_beat_idx: usize,
    pub us_to_next: usize,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TimeStatus {
    pub h: usize,
    pub m: usize,
    pub s: usize,
    pub f: usize,
    pub fp: usize,
}

impl Display for TimeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0>2}:{:0>2}:{:0>2}:{:0>2}",
            self.h, self.m, self.s, self.f
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceStatus {
    BeatStatus(BeatStatus),
    TimeStatus(TimeStatus),
    PlaybackStatus(PlaybackStatus),
    None,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStatus {
    pub sources: Vec<AudioSourceStatus>,
    pub running: bool,
    pub beat_idx: usize,
    pub next_beat_idx: usize,
    pub us_to_next_beat: usize,
    pub time: TimeStatus,
    pub system_time_us: u64,
    pub cue_idx: usize,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TimecodeInstant {
    pub frame_rate: usize,
    pub h: i16,
    pub m: i16,
    pub s: i16,
    pub f: i16,
    pub frame_progress: u16,
}

impl PartialEq for TimecodeInstant {
    fn eq(&self, other: &TimecodeInstant) -> bool {
        self.f == other.f && self.s == other.s && self.m == other.m && self.h == other.h
    }
}

impl Display for TimecodeInstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}:{}", self.h, self.m, self.s, self.f)
    }
}

impl TimecodeInstant {
    pub fn new(frame_rate: usize) -> TimecodeInstant {
        TimecodeInstant {
            frame_rate,
            ..Default::default()
        }
    }

    pub fn add_progress(&mut self, progress: u16) {
        let prog_of = self.frame_progress as u32 + progress as u32;
        self.frame_progress = (prog_of % 65536) as u16;
        if prog_of >= 65536 {
            self.f += 1
        }
        self.propagate();
    }
    pub fn add_us(&mut self, time_us: u64) {
        self.f += (time_us * self.frame_rate as u64 / 1000000) as i16;
        self.propagate();
    }
    pub fn sub_us(&mut self, time_us: u64) {
        self.f -= (time_us * self.frame_rate as u64 / 1000000) as i16;
        self.propagate();
    }

    pub fn set_time(&mut self, h: usize, m: usize, s: usize, f: usize) {
        self.h = h as i16;
        self.m = m as i16;
        self.s = s as i16;
        self.f = f as i16;
    }

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
            let mut time = time_const.clone();
            time.add_us(i);
            time.sub_us(i);
            assert_eq!(time, time_const, "Failed with {}us ({} s)", i, i / 1000000);
        }
    }
}

#[derive(Default)]
pub struct CombinedStatus {
    pub process_status: ProcessStatus,
    pub cue: Cue,
    pub show: Show,
    pub network_status: NetworkStatus,
    pub jack_status: JACKStatus,
}
