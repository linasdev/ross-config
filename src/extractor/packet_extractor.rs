use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::{ReferenceValue, Value};

#[repr(C)]
#[derive(Debug)]
pub struct PacketExtractor {}

impl PacketExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for PacketExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Value<'a> {
        Value::Reference(ReferenceValue::Packet(packet))
    }
}
