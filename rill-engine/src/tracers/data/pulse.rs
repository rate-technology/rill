use crate::tracers::tracer::Tracer;
use derive_more::{Deref, DerefMut};
use rill_protocol::data::pulse::{PulseEvent, PulseMetric};
use rill_protocol::io::provider::{Description, Path, StreamType};
use std::time::SystemTime;

/// Sends metrics as `pulse` that can change value to any.
#[derive(Debug, Deref, DerefMut, Clone)]
pub struct PulseTracer {
    tracer: Tracer<PulseMetric>,
}

impl PulseTracer {
    /// Creates a new `Pulse` tracer.
    pub fn new(path: Path) -> Self {
        let info = format!("{} pulse", path);
        let description = Description {
            path,
            info,
            stream_type: StreamType::PulseStream,
        };
        let tracer = Tracer::new(description, None);
        Self { tracer }
    }

    /// Increments the value by the specific delta.
    pub fn inc(&self, delta: f64, timestamp: Option<SystemTime>) {
        let data = PulseEvent::Increment(delta);
        self.tracer.send(data, timestamp);
    }

    /// Decrements the value by the specific delta.
    pub fn dec(&self, delta: f64, timestamp: Option<SystemTime>) {
        let data = PulseEvent::Decrement(delta);
        self.tracer.send(data, timestamp);
    }

    /// Set the value.
    pub fn set(&self, new_value: f64, timestamp: Option<SystemTime>) {
        let data = PulseEvent::Set(new_value);
        self.tracer.send(data, timestamp);
    }
}