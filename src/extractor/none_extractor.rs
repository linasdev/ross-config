use ross_protocol::packet::Packet;

use crate::extractor::Extractor;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct NoneExtractor {}

impl NoneExtractor {
    pub fn new() -> Self {
        NoneExtractor {}
    }
}

impl Extractor for NoneExtractor {
    fn extract<'a>(&self, _packet: &'a Packet) -> ExtractorValue<'a> {
        ExtractorValue::None
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
    fn none_extractor_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                                   // transmitter_address
            0x23,                                                   // transmitter_address
            0x45,                                                   // channel
            0x67,                                                   // brightness
        ];

        let extractor = NoneExtractor::new();

        assert_eq!(extractor.extract(&packet), ExtractorValue::None);
    }
}
