use std::{
    fmt::{self, Display},
    slice::Iter,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Beat {
    pub count: u8,
    pub bar_number: usize,
    pub length: usize,
    pub events: Vec<BeatEvent>,
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
            events: vec![],
        }
    }

    pub fn events_filter<F>(&self, filter: F) -> Vec<BeatEvent>
    where
        F: Fn(&BeatEvent) -> bool,
    {
        self.events.iter().filter(|e| filter(e)).cloned().collect()
    }

    pub fn tempo(&self) -> usize {
        (60000000.0 / self.length as f32).round() as usize
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum JumpRequirement {
    JumpModeOn,
    JumpModeOff,
    None,
}

impl Display for JumpRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpRequirement::JumpModeOn => write!(f, "VLT On"),
            JumpRequirement::JumpModeOff => write!(f, "VLT Off"),
            JumpRequirement::None => write!(f, "None"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum JumpModeChange {
    SetOn,
    SetOff,
    Toggle,
    None,
}

impl Display for JumpModeChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpModeChange::SetOn => write!(f, "Set VLT"),
            JumpModeChange::SetOff => write!(f, "Trip VLT"),
            JumpModeChange::Toggle => write!(f, "Toggle VLT"),
            JumpModeChange::None => write!(f, "None"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub enum BeatEvent {
    JumpEvent {
        destination: usize,
        requirement: JumpRequirement,
        when_jumped: JumpModeChange,
        when_passed: JumpModeChange,
    },
    TempoChangeEvent {
        tempo: usize,
    },
    GradualTempoChangeEvent {
        start_tempo: usize,
        end_tempo: usize,
        length: usize,
    },
    PlaybackEvent {
        channel_idx: usize,
        clip_idx: usize,
        sample: i32,
    },
    PlaybackStopEvent {
        channel_idx: usize,
    },
    TimecodeEvent {
        h: usize,
        m: usize,
        s: usize,
        f: usize,
    },
    RehearsalMarkEvent {
        label: String,
    },
}

impl BeatEvent {
    pub fn get_name(&self) -> &str {
        match self {
            BeatEvent::JumpEvent { .. } => "Jump",
            BeatEvent::TempoChangeEvent { .. } => "Tempo Change",
            BeatEvent::GradualTempoChangeEvent { .. } => "Gradual Tempo Change",
            BeatEvent::PlaybackEvent { .. } => "Playback",
            BeatEvent::PlaybackStopEvent { .. } => "Playback Stop",
            BeatEvent::TimecodeEvent { .. } => "Timecode",
            BeatEvent::RehearsalMarkEvent { .. } => "Rehearsal Mark",
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cue {
    pub metadata: CueMetadata,
    pub beats: Vec<Beat>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct CueMetadata {
    pub name: String,
    pub human_ident: String,
}

impl Default for Cue {
    fn default() -> Cue {
        Cue::empty()
    }
}

impl Cue {
    //    pub fn from_file(path: &str) -> Cue {
    //        return Cue::example();
    //    }

    pub fn empty() -> Cue {
        Cue {
            beats: vec![],
            metadata: CueMetadata::default(),
        }
    }
    pub fn example() -> Cue {
        let mut br = Cue {
            beats: vec![],
            metadata: CueMetadata::default(),
        };
        for i in 0..100 {
            br.beats.push(Beat {
                count: i % 4 + 1,
                bar_number: i as usize / 4 + 1,
                length: 500,
                events: vec![],
            });
        }
        br.beats[0].events.push(BeatEvent::PlaybackEvent {
            channel_idx: 0,
            clip_idx: 0,
            sample: 0,
        });
        br
    }

    pub fn example_loop() -> Cue {
        let mut br = Cue {
            beats: vec![],
            metadata: CueMetadata::default(),
        };
        for i in 0..8 {
            br.beats.push(Beat {
                count: i % 4 + 1,
                bar_number: i as usize / 4 + 1,
                length: 500,
                events: if i == 3 {
                    vec![BeatEvent::JumpEvent {
                        destination: 0,
                        requirement: JumpRequirement::None,
                        when_jumped: JumpModeChange::None,
                        when_passed: JumpModeChange::None,
                    }]
                } else {
                    vec![]
                },
            });
        }
        br.beats[0].events.push(BeatEvent::PlaybackEvent {
            channel_idx: 0,
            clip_idx: 0,
            sample: 0,
        });
        br
    }

    pub fn get_beat(&self, idx: usize) -> Option<Beat> {
        if self.beats.is_empty() || self.beats.len() <= idx {
            return None;
        }
        Some(self.beats[idx].clone())
    }

    pub fn get_beats(&self) -> Vec<Beat> {
        self.beats.clone()
    }

    pub fn reorder_numbers(&mut self) {
        if self.beats.is_empty() {
            return;
        }
        let mut bar = if self.beats[0].bar_number == 0 { 0 } else { 1 };
        let mut count = 1;
        let mut prev_bar = bar;
        for beat in &mut self.beats {
            if prev_bar != beat.bar_number || (beat.count == 1 && prev_bar > 1) {
                count = 1;
                bar += 1;
            }

            prev_bar = beat.bar_number;

            beat.bar_number = bar;
            beat.count = count;

            count += 1;
        }
    }

    pub fn recalculate_tempo_changes(&mut self) {
        let mut beat_length = 1000000 * 60 / 120;
        let mut beats_left_in_change = 0;
        let mut accelerator: f32 = 0.0;
        for beat in &mut self.beats {
            if let Some(BeatEvent::TempoChangeEvent { tempo }) = beat
                .events_filter(|f| matches!(f, BeatEvent::TempoChangeEvent { .. }))
                .get(0)
            {
                beat_length = 1000000 * 60 / tempo;
                accelerator = 0.0;
            }
            if let Some(BeatEvent::GradualTempoChangeEvent {
                start_tempo,
                end_tempo,
                length,
            }) = beat
                .events_filter(|f| matches!(f, BeatEvent::GradualTempoChangeEvent { .. }))
                .get(0)
            {
                beat_length = 1000000 * 60 / start_tempo;
                accelerator = (60000000.0 / *end_tempo as f32 - 60000000.0 / *start_tempo as f32)
                    / *length as f32;
                beats_left_in_change = *length;
            }
            beat.length = beat_length;
            beat_length = (beat_length as f32 + accelerator).round() as usize;
            beats_left_in_change = beats_left_in_change.saturating_sub(1);
            if beats_left_in_change == 0 {
                accelerator = 0.0;
            }
        }
    }
}
