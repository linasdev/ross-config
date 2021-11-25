use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct EventProducerAddressExtractor {}

impl EventProducerAddressExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for EventProducerAddressExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> ExtractorValue<'a> {
        if packet.data.len() < 4 {
            panic!("Wrong packet format provided for event producer address extractor.");
        }

        ExtractorValue::U16(u16::from_be_bytes(packet.data[2..=3].try_into().unwrap()))
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
    fn event_code_extractor_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                   // transmitter_address
            0x23,                                                   // transmitter_address
            0x45,                                                   // channel
            0x67,                                                   // brightness
        ];

        let extractor = EventProducerAddressExtractor::new();

        assert_eq!(
            extractor.extract(&packet),
            ExtractorValue::U16(0x0123),
        );
    }

    #[test]
    #[should_panic(expected = "Wrong packet format provided for event producer address extractor.")]
    fn event_code_extractor_wrong_format_test() {
        let mut packet = PACKET;
        packet.data = vec![((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8];

        let extractor = EventProducerAddressExtractor::new();

        extractor.extract(&packet);
    }
}
