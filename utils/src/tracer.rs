//! Tracer is a common trait that provide a way to log inside a library
//! Without force the user to use a library that is provided with the crate.
use std::fmt;

use log::{debug, info};

pub enum TraceLevel {
    Debug,
    Info,
}

pub trait Tracer {
    fn new() -> Self
    where
        Self: Sized;

    /// Generic log call to log a message in a specified level.
    fn log(&self, level: TraceLevel, msg: &String) {
        match level {
            TraceLevel::Debug => self.debug(msg),
            TraceLevel::Info => self.info(msg),
        }
    }

    fn debug(&self, msg: &String);

    fn info(&self, msg: &String);
}

impl fmt::Debug for dyn Tracer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "tracer");
        Ok(())
    }
}

// TODO: make this optional.
pub struct DefLogTracer;

impl Tracer for DefLogTracer {
    fn new() -> Self
    where
        Self: Sized,
    {
        DefLogTracer {}
    }

    fn debug(&self, msg: &String) {
        debug!("{}", msg);
    }

    fn info(&self, msg: &String) {
        info!("{}", msg);
    }
}

pub struct DummyTracer;

impl Tracer for DummyTracer {
    fn new() -> Self
    where
        Self: Sized,
    {
        DummyTracer {}
    }

    fn debug(&self, _msg: &String) {}

    fn info(&self, _msg: &String) {}
}
