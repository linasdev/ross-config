use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError};
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
    fn extract<'a>(&self, _packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        Ok(ExtractorValue::None)
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
}
