use crate::str::{String32, String8};
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

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

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, PartialOrd, Ord, Eq, Copy)]
pub enum JumpRequirement {
    JumpModeOn,
    JumpModeOff,
    None,
}

impl fmt::Display for JumpRequirement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpRequirement::JumpModeOn => write!(f, "VLT On"),
            JumpRequirement::JumpModeOff => write!(f, "VLT Off"),
            JumpRequirement::None => write!(f, "None"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default, PartialOrd, Ord, Eq, Copy)]
pub enum JumpModeChange {
    SetOn,
    SetOff,
    Toggle,
    #[default]
    None,
}

impl fmt::Display for JumpModeChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JumpModeChange::SetOn => write!(f, "Set VLT"),
            JumpModeChange::SetOff => write!(f, "Trip VLT"),
            JumpModeChange::Toggle => write!(f, "Toggle VLT"),
            JumpModeChange::None => write!(f, "None"),
        }
    }
}
impl JumpModeChange {
    pub fn vlt(&self, vlt: bool) -> bool {
        match self {
            JumpModeChange::SetOn => true,
            JumpModeChange::SetOff => false,
            JumpModeChange::Toggle => !vlt,
            JumpModeChange::None => vlt,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, PartialOrd, Ord, Eq, Copy)]
pub enum PauseEventBehaviour {
    Hold,
    RestartBeat,
    RestartCue,
    NextCue,
    Jump { destination: u16 },
}

impl fmt::Display for PauseEventBehaviour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PauseEventBehaviour::Hold => {
                write!(f, "Hold")
            }
            PauseEventBehaviour::RestartBeat => {
                write!(f, "Restart beat")
            }
            PauseEventBehaviour::RestartCue => {
                write!(f, "Restart cue")
            }
            PauseEventBehaviour::NextCue => {
                write!(f, "Next cue")
            }
            PauseEventBehaviour::Jump { .. } => {
                write!(f, "Jump to beat")
            }
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct BeatEventContainer {
    location: u16,
    event: Option<BeatEvent>,
}

impl Default for BeatEventContainer {
    fn default() -> Self {
        Self {
            location: u16::MAX,
            event: None,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Ord, PartialOrd, Eq, Copy)]
pub enum BeatEvent {
    JumpEvent {
        destination: u16,
        requirement: JumpRequirement,
        when_jumped: JumpModeChange,
        when_passed: JumpModeChange,
    },
    TempoChangeEvent {
        tempo: u16,
    },
    GradualTempoChangeEvent {
        start_tempo: u16,
        end_tempo: u16,
        length: u16,
    },
    PlaybackEvent {
        channel_idx: u16,
        clip_idx: u16,
        sample: i32,
    },
    PlaybackStopEvent {
        channel_idx: u16,
    },
    TimecodeEvent {
        h: u8,
        m: u8,
        s: u8,
        f: u8,
    },
    RehearsalMarkEvent {
        label: String8,
    },
    PauseEvent {
        behaviour: PauseEventBehaviour,
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
            BeatEvent::PauseEvent { .. } => "Pause Event",
        }
    }
}

const CUE_LENGTH: usize = 512;
const EVENT_SLOTS: usize = 64;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cue {
    pub metadata: CueMetadata,
    #[serde(with = "BigArray")]
    pub beats: [Beat; CUE_LENGTH],
    #[serde(with = "BigArray")]
    pub events: [BeatEventContainer; EVENT_SLOTS],
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CueSkeleton {
    pub metadata: CueMetadata,
    #[serde(with = "BigArray")]
    pub events: [BeatEventContainer; EVENT_SLOTS],
}

#[derive(Clone, Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct CueMetadata {
    pub name: String32,
    pub human_ident: String32,
}

impl Default for Cue {
    fn default() -> Cue {
        Cue::empty()
    }
}

impl Cue {
    pub fn empty() -> Cue {
        Cue {
            events: [BeatEventContainer::default(); EVENT_SLOTS],
            beats: [Beat::empty(); CUE_LENGTH],
            metadata: CueMetadata::default(),
        }
    }
    pub fn example() -> Cue {
        let mut br = Cue::empty();
        for i in 0..100 {
            br.beats[i] = Beat {
                count: i as u8 % 4 + 1,
                bar_number: i as u8 / 4 + 1,
                length: 500,
            };
        }
        br.events[0] = BeatEventContainer {
            location: 0,
            event: Some(BeatEvent::PlaybackEvent {
                channel_idx: 0,
                clip_idx: 0,
                sample: 0,
            }),
        };
        br
    }

    pub fn example_loop() -> Cue {
        let mut br = Cue::empty();
        for i in 0..8 {
            br.beats[i] = Beat {
                count: i as u8 % 4 + 1,
                bar_number: i as u8 / 4 + 1,
                length: 500,
            };
        }
        br.events[0] = BeatEventContainer {
            location: 0,
            event: Some(BeatEvent::PlaybackEvent {
                channel_idx: 0,
                clip_idx: 0,
                sample: 0,
            }),
        };
        br.events[1] = BeatEventContainer {
            location: 3,
            event: Some(BeatEvent::JumpEvent {
                destination: 0,
                requirement: JumpRequirement::None,
                when_jumped: JumpModeChange::None,
                when_passed: JumpModeChange::None,
            }),
        };
        br
    }

    pub fn get_beat(&self, idx: u16) -> Option<Beat> {
        if self.beats[idx as usize].length == 0 || CUE_LENGTH <= idx as usize {
            return None;
        }
        Some(self.beats[idx as usize])
    }

    pub fn get_beats(&self) -> [Beat; CUE_LENGTH] {
        self.beats
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

    pub fn sort_events(&mut self) {
        self.events.sort_by_key(|o| o.location);
    }

    pub fn get_cursor(&mut self) -> EventCursor<'_> {
        self.sort_events();
        EventCursor::new(self)
    }

    pub fn recalculate_tempo_changes(&mut self) {
        let mut beat_length: u32 = 1000000 * 60 / 120;
        let mut beats_left_in_change = 0;
        let mut accelerator: f32 = 0.0;

        let mut new_beats = self.beats;
        let mut cursor = self.get_cursor();

        for beat in &mut new_beats {
            if let Some(BeatEvent::TempoChangeEvent { tempo }) = cursor.get().event {
                cursor.step();
                beat_length = 1000000 * 60 / tempo as u32;
                accelerator = 0.0;
            }
            if let Some(BeatEvent::GradualTempoChangeEvent {
                start_tempo,
                end_tempo,
                length,
            }) = cursor.get().event
            {
                cursor.step();
                beat_length = 1000000 * 60 / start_tempo as u32;
                accelerator = (60000000.0 / end_tempo as f32 - 60000000.0 / start_tempo as f32)
                    / length as f32;
                beats_left_in_change = length;
            }
            beat.length = beat_length;
            beat_length = (beat_length as f32 + accelerator).round() as u32;
            beats_left_in_change = beats_left_in_change.saturating_sub(1);
            if beats_left_in_change == 0 {
                accelerator = 0.0;
            }
        }

        drop(cursor);
        self.beats = new_beats;
    }
}

pub struct EventCursor<'a> {
    cursor: u8,
    cue: &'a Cue,
}

impl<'a> EventCursor<'a> {
    fn new(cue: &'a Cue) -> EventCursor<'a> {
        Self { cursor: 0, cue }
    }

    fn seek(&mut self, location: u16) {
        if self.location() > location {
            self.cursor = 0;
        }
        while self.location() < location && self.cursor < EVENT_SLOTS as u8 {
            self.cursor += 1;
        }
    }

    fn location(&self) -> u16 {
        self.cue.events[self.cursor as usize].location
    }

    fn get(&mut self) -> BeatEventContainer {
        self.cue.events[self.cursor as usize]
    }

    fn step(&mut self) {
        self.cursor += (self.cursor + 1).min(EVENT_SLOTS as u8);
    }
}
