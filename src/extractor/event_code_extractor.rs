use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError};
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct EventCodeExtractor {}

impl EventCodeExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for EventCodeExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        if packet.data.len() < 2 {
            Err(ExtractorError::PacketTooShort)
        } else {
            Ok(ExtractorValue::U16(u16::from_be_bytes(
                packet.data[0..=1].try_into().unwrap(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: Vec::new(),
    };

    #[test]
    fn correct_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            0x00, // event code
            0x00, // event code
        ];

        let extractor = EventCodeExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::U16(0x0000)));
    }

    #[test]
    fn wrong_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            0x00, // event code
                 // missing byte
        ];

        let extractor = EventCodeExtractor::new();

        assert_eq!(
            extractor.extract(&packet),
            Err(ExtractorError::PacketTooShort)
        );
    }
}
