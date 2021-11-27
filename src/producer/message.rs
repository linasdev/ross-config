use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::message::{MessageEvent, MessageValue};
use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError};
use crate::state_manager::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct MessageProducer {
    receiver_address: u16,
    code: u16,
    value: MessageValue,
}

impl MessageProducer {
    pub fn new(receiver_address: u16, code: u16, value: MessageValue) -> Self {
        Self {
            receiver_address,
            code,
            value,
        }
    }
}

impl Producer for MessageProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let event = MessageEvent {
            receiver_address: self.receiver_address,
            transmitter_address: device_address,
            code: self.code,
            value: self.value.clone(),
        };

        Ok(Some(event.to_packet()))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;

    use ross_protocol::event::event_code::MESSAGE_EVENT_CODE;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: vec![],
    };

    #[test]
    fn test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((MESSAGE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((MESSAGE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                     // transmitter address
            0x00,                                     // transmitter address
            0x01,                                     // code
            0x23,                                     // code
            0x02,                                     // value
            0x00,                                     // value
            0x00,                                     // value
            0x00,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
            0xff,                                     // value
        ];

        let state_manager = StateManager::new();

        let producer = MessageProducer::new(0xabab, 0x0123, MessageValue::U32(0xffff_ffff));

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }
}
