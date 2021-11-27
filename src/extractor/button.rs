extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::button::{ButtonPressedEvent, ButtonReleasedEvent};
use ross_protocol::packet::Packet;

use crate::extractor::{Extractor, ExtractorError, BUTTON_INDEX_EXTRACTOR_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct ButtonIndexExtractor {}

impl ButtonIndexExtractor {
    pub fn new() -> Self {
        Self {}
    }
}

impl Extractor for ButtonIndexExtractor {
    fn extract<'a>(&self, packet: &'a Packet) -> Result<ExtractorValue<'a>, ExtractorError> {
        if let Ok(event) = ButtonPressedEvent::try_from_packet(packet) {
            return Ok(ExtractorValue::U8(event.index));
        }

        match ButtonReleasedEvent::try_from_packet(packet) {
            Ok(event) => Ok(ExtractorValue::U8(event.index)),
            Err(err) => Err(ExtractorError::ConvertPacketError(err)),
        }
    }

    fn get_code(&self) -> u16 {
        BUTTON_INDEX_EXTRACTOR_CODE
    }
}

impl Serialize for ButtonIndexExtractor {
    fn serialize(&self) -> Vec<u8> {
        vec![]
    }
}

impl TryDeserialize for ButtonIndexExtractor {
    fn try_deserialize(_data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        Ok(Box::new(Self {}))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;

    use ross_protocol::event::event_code::{BUTTON_PRESSED_EVENT_CODE, BUTTON_RELEASED_EVENT_CODE};

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: vec![],
    };

    #[test]
    fn pressed_correct_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                            // button address
            0x23,                                            // button address
            0x45,                                            // index
        ];

        let extractor = ButtonIndexExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::U8(0x45)),);
    }

    #[test]
    fn pressed_wrong_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BUTTON_PRESSED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_PRESSED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                            // button address
            0x23,                                            // button address
                                                             // missing byte
        ];

        let extractor = ButtonIndexExtractor::new();

        assert!(matches!(
            extractor.extract(&packet),
            Err(ExtractorError::ConvertPacketError(_))
        ));
    }

    #[test]
    fn released_correct_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // button address
            0x23,                                             // button address
            0x45,                                             // index
        ];

        let extractor = ButtonIndexExtractor::new();

        assert_eq!(extractor.extract(&packet), Ok(ExtractorValue::U8(0x45)),);
    }

    #[test]
    fn released_wrong_format_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BUTTON_RELEASED_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BUTTON_RELEASED_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x01,                                             // button address
            0x23,                                             // button address
                                                              // missing byte
        ];

        let extractor = ButtonIndexExtractor::new();

        assert!(matches!(
            extractor.extract(&packet),
            Err(ExtractorError::ConvertPacketError(_))
        ));
    }

    #[test]
    fn serialize_test() {
        let extractor = ButtonIndexExtractor::new();

        let expected_data = vec![];

        assert_eq!(extractor.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![];

        let extractor = Box::new(ButtonIndexExtractor::new());

        assert_eq!(ButtonIndexExtractor::try_deserialize(&data), Ok(extractor));
    }
}
