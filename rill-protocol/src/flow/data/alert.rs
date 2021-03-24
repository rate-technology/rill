use crate::flow::data::{Flow, TimedEvent};
use crate::io::provider::StreamType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AlertFlow;

impl Flow for AlertFlow {
    type State = AlertState;
    type Event = AlertEvent;

    fn stream_type() -> StreamType {
        StreamType::from("rillrate.alert.v0")
    }

    fn apply(&self, _state: &mut Self::State, _event: TimedEvent<Self::Event>) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertState {}

#[allow(clippy::new_without_default)]
impl AlertState {
    pub fn new() -> Self {
        Self {}
    }
}

pub type AlertDelta = Vec<TimedEvent<AlertEvent>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertEvent {
    pub msg: String,
}