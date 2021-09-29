use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, Value};

#[repr(C)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct EventCodeExtractor {}

impl EventCodeExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

#[cfg_attr(feature = "std", typetag::serde(name = "event_code_extractor"))]
impl Extractor for EventCodeExtractor {
    fn extract(&self, packet: &Packet) -> Value {
        if packet.data.len() < 2 {
            panic!("Wrong packet format provided for event code extractor.");
        }

        Value::U16(u16::from_be_bytes(packet.data[0..=1].try_into().unwrap()))
    }
}
