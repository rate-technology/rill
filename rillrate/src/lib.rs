//! Dynamic tracing system that tends to be real-time.

#![warn(missing_docs)]

mod config;
mod env;
mod supervisor;
mod tracers;

pub use rill;
pub use rill_export;
pub use rill_protocol as protocol;
pub use tracers::*;

use anyhow::Error;
use meio::thread::ScopedRuntime;

/// The tracer.
pub struct RillRate {
    _scoped: ScopedRuntime,
}

impl RillRate {
    /// Creates an instance of `RillRate` tracer using environment vars.
    pub fn from_env(app_name: impl ToString) -> Result<Self, Error> {
        use supervisor::RillRate;
        let actor = RillRate::new(app_name.to_string());
        let _scoped = meio::thread::spawn(actor)?;
        Ok(Self { _scoped })
    }
}
