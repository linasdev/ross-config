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

#[repr(C)]
#[derive(Debug)]
pub struct EventProducerAddressExtractor {}

impl EventProducerAddressExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for EventProducerAddressExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        if packet.data.len() < 4 {
            Err(ExtractorError::PacketTooShort)
        } else {
            Ok(ExtractorValue::U16(u16::from_be_bytes(
                packet.data[2..=3].try_into().unwrap(),
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: vec![],
    };

    #[test]
    fn event_code_test() {
        let mut packet = PACKET;
        packet.data = vec![
            0x00, // event code
            0x00, // event code
        ];

        let extractor = EventCodeExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::U16(0x0000)));
    }

    #[test]
    fn event_code_wrong_format_test() {
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

    #[test]
    fn event_producer_address_test() {
        let mut packet = PACKET;
        packet.data = vec![
            0x00, // event code
            0x00, // event code
            0x01, // transmitter address
            0x23, // transmitter address
        ];

        let extractor = EventProducerAddressExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::U16(0x0123)));
    }

    #[test]
    fn event_producer_address_wrong_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            0x00, // event code
            0x00, // event code
            0x01, // transmitter address
                  // missing byte
        ];

        let extractor = EventProducerAddressExtractor::new();

        assert_eq!(
            extractor.extract(&packet),
            Err(ExtractorError::PacketTooShort)
        );
    }
}
