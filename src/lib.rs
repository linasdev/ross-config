#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::vec::Vec;
use alloc::collections::BTreeMap;

use crate::event_processor::EventProcessor;

pub mod extractor;
pub mod filter;
pub mod matcher;
pub mod producer;
pub mod state;
pub mod event_processor;

#[cfg(test)]
mod test;

#[derive(Debug, PartialEq)]
pub enum Value {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
}

#[derive(Debug)]
pub struct Config {
    initial_state: BTreeMap<u32, Value>,
    event_processors: Vec<EventProcessor>,
}
