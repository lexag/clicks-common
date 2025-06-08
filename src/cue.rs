use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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
    VampEvent {
        length: usize,
    },
    PlaybackEvent {
        sample: u64,
    },
    TimecodeEvent {
        h: usize,
        m: usize,
        s: usize,
        f: usize,
    },
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Cue {
    name: String,
    human_ident: String,
    beats: Vec<Beat>,
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
            name: String::new(),
            human_ident: String::new(),
        }
    }
    pub fn example() -> Cue {
        let mut br = Cue {
            beats: vec![],
            name: "example cue".to_string(),
            human_ident: "1.2.3".to_string(),
        };
        for i in 0..100 {
            br.beats.push(Beat {
                count: i % 4 + 1,
                bar_number: i as usize / 4 + 1,
                length: 500,
                events: vec![],
            });
        }
        br.beats[0]
            .events
            .push(BeatEvent::PlaybackEvent { sample: 0 });
        return br;
    }

    pub fn example_loop() -> Cue {
        let mut br = Cue {
            beats: vec![],
            name: "looping cue".to_string(),
            human_ident: "4.5.6".to_string(),
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
        br.beats[0]
            .events
            .push(BeatEvent::PlaybackEvent { sample: 0 });
        return br;
    }

    pub fn get_beat(&self, idx: usize) -> Result<Beat, ()> {
        if self.beats.len() == 0 {
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
