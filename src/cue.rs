use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Default)]
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
}

#[derive(Clone, Serialize, Deserialize)]
pub enum BeatEvent {
    JumpEvent {
        destination: usize,
    },
    VoltaEvent {
        destination: usize,
    },
    RepeatStartEvent,
    TempoChangeEvent {
        tempo: usize,
    },
    VampEvent {
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
            BeatEvent::JumpEvent { .. } => "Jump Event",
            BeatEvent::VoltaEvent { .. } => "Volta Event",
            BeatEvent::RepeatStartEvent => "Repeat Start Event",
            BeatEvent::TempoChangeEvent { .. } => "Tempo Change Event",
            BeatEvent::VampEvent { .. } => "Vamp Event",
            BeatEvent::PlaybackEvent { .. } => "Playback Event",
            BeatEvent::PlaybackStopEvent { .. } => "Playback Stop Event",
            BeatEvent::TimecodeEvent { .. } => "Timecode Event",
            BeatEvent::RehearsalMarkEvent { .. } => "Rehearsal Mark Event",
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cue {
    pub metadata: CueMetadata,
    pub beats: Vec<Beat>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
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
        return br;
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
                    vec![BeatEvent::VampEvent { length: 4 }]
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
        return br;
    }

    pub fn get_beat(&self, idx: usize) -> Result<Beat, ()> {
        if self.beats.is_empty() {
            return Err(());
        }
        if self.beats.len() <= idx {
            return Err(());
        }
        return Ok(self.beats[idx].clone());
    }

    pub fn get_beats(&self) -> Vec<Beat> {
        return self.beats.clone();
    }
}
