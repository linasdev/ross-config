use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::Value;

#[repr(C)]
#[derive(Debug)]
pub struct EventProducerAddressExtractor {}

impl EventProducerAddressExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for EventProducerAddressExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Value<'a> {
        if packet.data.len() < 4 {
            panic!("Wrong packet format provided for event producer address extractor.");
        }

        Value::U16(u16::from_be_bytes(packet.data[2..=3].try_into().unwrap()))
    }
}
