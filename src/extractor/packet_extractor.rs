use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError};
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
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
}
