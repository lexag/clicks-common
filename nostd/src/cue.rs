use crate::{
    beat::Beat,
    event::{BeatEvent, BeatEventContainer, JumpModeChange, JumpRequirement},
    eventcursor::EventCursor,
    str::String32,
};
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Cue {
    pub metadata: CueMetadata,
    #[serde(with = "BigArray")]
    pub beats: [Beat; Cue::CUE_LENGTH],
    #[serde(with = "BigArray")]
    pub events: [BeatEventContainer; Self::EVENT_SLOTS],
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct CueSkeleton {
    pub metadata: CueMetadata,
    #[serde(with = "BigArray")]
    pub events: [BeatEventContainer; Cue::EVENT_SLOTS],
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
    pub const CUE_LENGTH: usize = 512;
    pub const EVENT_SLOTS: usize = 64;

    pub fn empty() -> Cue {
        Cue {
            events: [BeatEventContainer::default(); Self::EVENT_SLOTS],
            beats: [Beat::empty(); Self::CUE_LENGTH],
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
        if self.beats[idx as usize].length == 0 || Self::CUE_LENGTH <= idx as usize {
            return None;
        }
        Some(self.beats[idx as usize])
    }

    pub fn get_beats(&self) -> [Beat; Self::CUE_LENGTH] {
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
