use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm::BcmChangeBrightnessEvent;
use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug)]
pub struct BcmChangeBrightnessProducer {
    bcm_address: u16,
    channel: u8,
    brightness: u8,
}

impl BcmChangeBrightnessProducer {
    pub fn new(bcm_address: u16, channel: u8, brightness: u8) -> Self {
        Self {
            bcm_address,
            channel,
            brightness,
        }
    }
}

impl Producer for BcmChangeBrightnessProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let event = BcmChangeBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_address,
            channel: self.channel,
            brightness: self.brightness,
        };

        Ok(Some(event.to_packet()))
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct BcmChangeBrightnessStateProducer {
    bcm_address: u16,
    channel: u8,
    state_index: u32,
}

impl BcmChangeBrightnessStateProducer {
    pub fn new(bcm_address: u16, channel: u8, state_index: u32) -> Self {
        Self {
            bcm_address,
            channel,
            state_index,
        }
    }
}

impl Producer for BcmChangeBrightnessStateProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let current_value = *match state_manager.get_value(self.state_index) {
            Some(Value::U8(value)) => value,
            _ => return Err(ProducerError::WrongStateType),
        };

        let event = BcmChangeBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_address,
            channel: self.channel,
            brightness: current_value,
        };

        Ok(Some(event.to_packet()))
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
    fn change_brightness_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                   // transmitter address
            0x00,                                                   // transmitter address
            0x01,                                                   // channel
            0x02,                                                   // brightness
        ];

        let state_manager = StateManager::new();

        let producer = BcmChangeBrightnessProducer::new(PACKET.device_address, 0x01, 0x02);

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn change_brightness_state_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_CHANGE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                   // transmitter address
            0x00,                                                   // transmitter address
            0x01,                                                   // channel
            0x02,                                                   // brightness
        ];

        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::U8(0x02));

        let producer = BcmChangeBrightnessStateProducer::new(0xabab, 0x01, 0);

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }
}
