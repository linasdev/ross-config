extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError, EVENT_CODE_EXTRACTOR_CODE, EVENT_PRODUCER_ADDRESS_EXTRACTOR_CODE};
use crate::ExtractorValue;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
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

    fn get_code(&self) -> u16 {
        EVENT_CODE_EXTRACTOR_CODE
    }
}

impl Serialize for EventCodeExtractor {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for EventCodeExtractor {
    fn try_deserialize(_data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        Ok(Box::new(Self {}))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
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

    fn get_code(&self) -> u16 {
        EVENT_PRODUCER_ADDRESS_EXTRACTOR_CODE
    }
}

impl Serialize for EventProducerAddressExtractor {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for EventProducerAddressExtractor {
    fn try_deserialize(_data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        Ok(Box::new(Self {}))
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
    fn event_code_serialize_test() {
        let extractor = EventCodeExtractor::new();

        let expected_data = vec![];

        assert_eq!(extractor.serialize(), expected_data);
    }

    #[test]
    fn event_code_deserialize_test() {
        let data = vec![];

        let extractor = Box::new(EventCodeExtractor::new());

        assert_eq!(EventCodeExtractor::try_deserialize(&data), Ok(extractor));
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

    #[test]
    fn event_producer_address_serialize_test() {
        let extractor = EventProducerAddressExtractor::new();

        let expected_data = vec![];

        assert_eq!(extractor.serialize(), expected_data);
    }

    #[test]
    fn event_producer_address_deserialize_test() {
        let data = vec![];

        let extractor = Box::new(EventProducerAddressExtractor::new());

        assert_eq!(EventProducerAddressExtractor::try_deserialize(&data), Ok(extractor));
    }
}
