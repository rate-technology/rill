use super::{ConvertError, Delta, State, TimedEvent};
use crate::io::provider::{StreamDelta, StreamState, Timestamp};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CounterState {
    pub timestamp: Option<Timestamp>,
    pub value: f64,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            timestamp: None,
            value: 0.0,
        }
    }
}

impl TryFrom<StreamState> for CounterState {
    type Error = ConvertError;

    fn try_from(state: StreamState) -> Result<Self, ConvertError> {
        match state {
            StreamState::Counter(state) => Ok(state),
            _ => Err(ConvertError),
        }
    }
}

impl State for CounterState {
    type Event = CounterEvent;

    fn apply(&mut self, event: TimedEvent<Self::Event>) {
        match event.event {
            CounterEvent::Increment(delta) => {
                self.timestamp = Some(event.timestamp);
                self.value += delta;
            }
        }
    }

    fn wrap(events: Delta<Self::Event>) -> StreamDelta {
        StreamDelta::from(events)
    }

    fn try_extract(delta: StreamDelta) -> Result<Delta<Self::Event>, ConvertError> {
        delta.try_into()
    }
}

pub type CounterDelta = Vec<TimedEvent<CounterEvent>>;

impl TryFrom<StreamDelta> for CounterDelta {
    type Error = ConvertError;

    fn try_from(delta: StreamDelta) -> Result<Self, ConvertError> {
        match delta {
            StreamDelta::Counter(delta) => Ok(delta),
            _ => Err(ConvertError),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CounterEvent {
    Increment(f64),
}
