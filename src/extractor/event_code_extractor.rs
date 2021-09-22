use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, Value};

pub struct EventCodeExtractor {}

impl EventCodeExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for EventCodeExtractor {
    fn extract(&self, packet: &Packet) -> Value {
        Value::U16(u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()))
    }
}
