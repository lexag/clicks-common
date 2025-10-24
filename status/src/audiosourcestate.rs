#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AudioSourceState {
    BeatStatus(BeatState),
    TimeStatus(TimecodeInstant),
    PlaybackStatus(PlaybackState),
}
