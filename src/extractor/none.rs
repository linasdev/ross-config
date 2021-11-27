extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError, NONE_EXTRACTOR_CODE};
use crate::ExtractorValue;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct NoneExtractor {}

impl NoneExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for NoneExtractor {
    fn extract<'a>(&self, _packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        Ok(ExtractorValue::None)
    }

    fn get_code(&self) -> u16 {
        NONE_EXTRACTOR_CODE
    }
}

impl Serialize for NoneExtractor {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for NoneExtractor {
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
        packet.data = vec![];

        let extractor = NoneExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::None));
    }

    #[test]
    fn serialize_test() {
        let extractor = NoneExtractor::new();

        let expected_data = vec![];

        assert_eq!(extractor.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![];

        let extractor = Box::new(NoneExtractor::new());

        assert_eq!(NoneExtractor::try_deserialize(&data), Ok(extractor));
    }
}
