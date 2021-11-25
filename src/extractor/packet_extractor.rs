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

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    use ross_protocol::event::event_code::BCM_CHANGE_BRIGHTNESS_EVENT_CODE;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: Vec::new(),
    };

    #[test]
    fn packet_extractor_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                   // transmitter_address
            0x23,                                                   // transmitter_address
            0x45,                                                   // channel
            0x67,                                                   // brightness
        ];

        let extractor = PacketExtractor::new();

        assert_eq!(
            extractor.extract(&packet),
            Value::Reference(ReferenceValue::Packet(&packet))
        );
    }
}
