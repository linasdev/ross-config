#![no_std]

use serde::{Serialize, Deserialize};

use ross_protocol::packet::Packet;

pub mod config;
pub mod event_processor;
pub mod extractor;
pub mod filter;
pub mod matcher;
pub mod producer;
pub mod state;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum StateValue {
    U8(u8),
    U16(u16),
    U32(u32),
    Bool(bool),
}

#[derive(Debug, PartialEq)]
pub enum ExtractorValue<'a> {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
    Packet(&'a Packet),
}
