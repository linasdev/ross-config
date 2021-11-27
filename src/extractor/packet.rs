extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError, PACKET_EXTRACTOR_CODE};
use crate::ExtractorValue;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct PacketExtractor {}

impl PacketExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for PacketExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        Ok(ExtractorValue::Packet(packet))
    }

    fn get_code(&self) -> u16 {
        PACKET_EXTRACTOR_CODE
    }
}

impl Serialize for PacketExtractor {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for PacketExtractor {
    fn try_deserialize(_data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        Ok(Box::new(Self {}))
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
    fn test() {
        let mut packet = PACKET;
        packet.data = vec![
            0x00, // event code
            0x00, // event code
            0x01, // transmitter address
            0x23, // transmitter address
        ];

        let extractor = PacketExtractor::new();

        assert_eq!(
            extractor.extract(&packet),
            Ok(ExtractorValue::Packet(&packet))
        );
    }

    #[test]
    fn serialize_test() {
        let extractor = PacketExtractor::new();

        let expected_data = vec![];

        assert_eq!(extractor.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![];

        let extractor = Box::new(PacketExtractor::new());

        assert_eq!(PacketExtractor::try_deserialize(&data), Ok(extractor));
    }
}
