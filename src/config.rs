extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::event_processor::EventProcessor;
use crate::Value;

#[derive(Debug)]
pub struct Config {
    pub initial_state: BTreeMap<u32, Value>,
    pub event_processors: Vec<EventProcessor>,
}
