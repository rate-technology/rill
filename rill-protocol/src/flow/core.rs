use crate::encoding;
use crate::io::provider::{PackedAction, PackedDelta, PackedState, StreamType, Timestamp};
use anyhow::Error;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

// TODO: Move to the separate module
/// Requirements for a data fraction in a data flow.
pub trait DataFraction:
    DeserializeOwned + Serialize + Clone + fmt::Debug + Sync + Send + 'static
{
}

impl<T> DataFraction for T where
    T: DeserializeOwned + Serialize + Clone + fmt::Debug + Sync + Send + 'static
{
}

/// Immutable state of a data flow.
pub trait Flow: DataFraction {
    /// `ControlEvent` - that send from a client to a server
    type Action: DataFraction;

    /// `UpdateEvent` - that sent from a server to a client
    type Event: DataFraction;

    fn stream_type() -> StreamType;

    fn apply(&mut self, event: TimedEvent<Self::Event>);

    fn pack_state(&self) -> Result<PackedState, Error> {
        encoding::pack(self)
    }

    fn unpack_state(data: &PackedState) -> Result<Self, Error> {
        encoding::unpack(data)
    }

    fn pack_delta(delta: &[TimedEvent<Self::Event>]) -> Result<PackedDelta, Error> {
        encoding::pack(delta)
    }

    fn unpack_delta(data: &PackedDelta) -> Result<Vec<TimedEvent<Self::Event>>, Error> {
        encoding::unpack(data)
    }

    fn pack_action(action: &Self::Action) -> Result<PackedAction, Error> {
        encoding::pack(action)
    }

    fn unpack_action(data: &PackedAction) -> Result<Self::Action, Error> {
        encoding::unpack(data)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimedEvent<T> {
    pub timestamp: Timestamp,
    pub event: T,
}

impl<T> TimedEvent<T> {
    pub fn into_inner(self) -> T {
        self.event
    }
}

impl<T> Ord for TimedEvent<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

impl<T> PartialOrd for TimedEvent<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> PartialEq for TimedEvent<T> {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}

impl<T> Eq for TimedEvent<T> {}
