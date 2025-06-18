use crate::{command::ControlCommand, cue::Cue, show::Show, status::ProcessStatus};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum StatusMessageKind {
    ProcessStatus(Option<ProcessStatus>),
    CueStatus(Option<Cue>),
    ShowStatus(Option<Show>),
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ControlMessageKind {
    ControlCommand(ControlCommand),
    SubscribeRequest(SubscriberInfo),
    UnsubscribeRequest(SubscriberInfo),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SubscriberInfo {
    pub address: String,
    pub port: String,
    pub message_kinds: Vec<StatusMessageKind>,
}
