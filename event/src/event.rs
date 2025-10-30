use core::fmt;
use mem::str::String8;
use serde::{Deserialize, Serialize};

/// Conditional VLT requirement to perform a [EventDescription::JumpEvent].
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, PartialOrd, Ord, Eq, Copy)]
pub enum JumpRequirement {
    /// VLT must be on
    JumpModeOn,
    /// VLT must be off
    JumpModeOff,
    /// Jump regardless of VLT
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

/// How (if at all) to change VLT state, for example after a jump or on a request from client
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Default, PartialOrd, Ord, Eq, Copy)]
pub enum JumpModeChange {
    /// Set VLT on
    SetOn,
    /// Set VLT off
    SetOff,
    /// Toggle VLT: on -> off, off -> on
    Toggle,
    /// Do nothing: on -> on, off -> off
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
    /// What should the new VLT value be, considering self and the old VLT value
    pub fn vlt(&self, vlt: bool) -> bool {
        match self {
            JumpModeChange::SetOn => true,
            JumpModeChange::SetOff => false,
            JumpModeChange::Toggle => !vlt,
            JumpModeChange::None => vlt,
        }
    }
}

/// When pausing from a PauseEvent, what action to take to prepare for playback again
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, PartialOrd, Ord, Eq, Copy)]
pub enum PauseEventBehaviour {
    /// Pause and do nothing. Playback will resume exactly where stopped, which may have been in
    /// the middle of a beat.
    Hold,
    /// Move back to the beginning of the beat where the pause happened. Will always resume in time.
    RestartBeat,
    /// Move to the start of the first beat in the cue. Will always start in time.
    RestartCue,
    /// Move to the start of the first beat in the next cue after this one. Will always start in
    /// time.
    NextCue,
    /// Jump to the start of an arbitrary beat in this cue. Will always start in time.
    Jump {
        /// Beat idx to start at
        destination: u16,
    },
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

/// Event defines any event which happens on a specific beat in a cue.
/// All triggers, markers and similar are events.
///
/// Events can be unpopulated, in which case they have location = u16::MAX and event = None
#[derive(Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Event {
    /// Location (beat idx) of this event in the cue
    pub location: u16,
    /// Event descriptor with details
    pub event: Option<EventDescription>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            location: u16::MAX,
            event: None,
        }
    }
}

impl Event {
    /// Create a new event from a location and description
    pub fn new(location: u16, description: EventDescription) -> Self {
        Self {
            location,
            event: Some(description),
        }
    }

    /// Create a new null event, without location or event desciption
    pub fn null() -> Self {
        Self::default()
    }

    /// Is this event null (opposite of populated), i.e. is this event just an empty slot in the
    /// event table (true), or an actual event (false)?
    pub fn is_null(&self) -> bool {
        self.location == u16::MAX || self.event.is_none()
    }
}

/// EventDescription contains definitions for all event types, and the data they contain
/// All events are const-size to support uC communication, but must not be equal size to each
/// other. Guideline is about 128 bytes per event.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug, Ord, PartialOrd, Eq, Copy)]
pub enum EventDescription {
    /// When triggered: change the next beat pointer to this event's destination field.
    /// Can be conditional with [JumpRequirement] and can conditionally change VLT state with [JumpModeChange]
    JumpEvent {
        /// Beat idx to jump to
        destination: u16,
        /// Conditional VLT requirement required to jump
        requirement: JumpRequirement,
        /// When jumping, should VLT change?
        when_jumped: JumpModeChange,
        /// When passing without jumping, should VLT change?
        when_passed: JumpModeChange,
    },

    /// Marks a direct tempo change. Cosmetic during playback, as tempo comes from each beat's
    /// length property, but used for editing and recalculating tempo
    TempoChangeEvent {
        /// New tempo in BPM
        tempo: u16,
    },
    /// Marks a gradual tempo change. Cosmetic during playback, as tempo comes from each beat's
    /// length property, but used for editing and recalculating tempo
    GradualTempoChangeEvent {
        /// Old tempo (ramp start tempo) in BPM
        start_tempo: u16,
        /// New tempo (ramp end tempo) in BPM
        end_tempo: u16,
        /// Length of tempo ramp in beats
        length: u16,
    },

    /// When triggered: start playing a clip on a playback channel
    PlaybackEvent {
        /// targeted playback channel
        channel_idx: u16,
        /// playback clip idx
        clip_idx: u16,
        /// Clip start offset in samples
        sample: i32,
    },
    /// When triggered: stop playback on a channel
    PlaybackStopEvent {
        /// Targeted playback channel
        channel_idx: u16,
    },

    /// Marks an SMPTE LTC timestamp
    /// FIXME: timecode event should probably implement TimecodeInstant instead of separate time
    /// values
    TimecodeEvent {
        /// Hours
        h: u8,
        /// Minutes
        m: u8,
        /// Seconds
        s: u8,
        /// Frames
        f: u8,
    },

    /// Cosmetic marker for rehearsal marks
    RehearsalMarkEvent {
        /// Rehearsal mark label
        /// Can be empty.
        label: String8,
    },

    /// When triggered: pause transport, and execute the PauseEventBehaviour.
    PauseEvent {
        /// What to do after pausing
        behaviour: PauseEventBehaviour,
    },
}

impl EventDescription {
    /// Get human readable name of event type. Does not contain inner event data
    pub fn get_name(&self) -> &str {
        match self {
            EventDescription::JumpEvent { .. } => "Jump",
            EventDescription::TempoChangeEvent { .. } => "Tempo Change",
            EventDescription::GradualTempoChangeEvent { .. } => "Gradual Tempo Change",
            EventDescription::PlaybackEvent { .. } => "Playback",
            EventDescription::PlaybackStopEvent { .. } => "Playback Stop",
            EventDescription::TimecodeEvent { .. } => "Timecode",
            EventDescription::RehearsalMarkEvent { .. } => "Rehearsal Mark",
            EventDescription::PauseEvent { .. } => "Pause Event",
        }
    }
}
