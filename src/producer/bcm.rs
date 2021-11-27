extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm::BcmChangeBrightnessEvent;
use ross_protocol::packet::Packet;

use crate::producer::{
    Producer, ProducerError, BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE,
    BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE,
};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug, PartialEq)]
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

    fn get_code(&self) -> u16 {
        BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE
    }
}

impl Serialize for BcmChangeBrightnessProducer {
    fn serialize(&self) -> Vec<u8> {
        let bcm_address = self.bcm_address.to_be_bytes();

        vec![
            bcm_address[0],
            bcm_address[1],
            self.channel,
            self.brightness,
        ]
    }
}

impl TryDeserialize for BcmChangeBrightnessProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 4 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let bcm_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let channel = data[2];
        let brightness = data[3];

        Ok(Box::new(Self {
            bcm_address,
            channel,
            brightness,
        }))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
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

    fn get_code(&self) -> u16 {
        BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
    }
}

impl Serialize for BcmChangeBrightnessStateProducer {
    fn serialize(&self) -> Vec<u8> {
        let bcm_address = self.bcm_address.to_be_bytes();
        let state_index = self.state_index.to_be_bytes();

        vec![
            bcm_address[0],
            bcm_address[1],
            self.channel,
            state_index[0],
            state_index[1],
            state_index[2],
            state_index[3],
        ]
    }
}

impl TryDeserialize for BcmChangeBrightnessStateProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 7 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let bcm_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let channel = data[2];
        let state_index = u32::from_be_bytes(data[3..=6].try_into().unwrap());

        Ok(Box::new(Self {
            bcm_address,
            channel,
            state_index,
        }))
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
    fn change_brightness_serialize_test() {
        let producer = BcmChangeBrightnessProducer::new(0xabab, 0x01, 0x23);

        let expected_data = vec![0xab, 0xab, 0x01, 0x23];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn change_brightness_deserialize_test() {
        let data = vec![0xab, 0xab, 0x01, 0x23];

        let producer = Box::new(BcmChangeBrightnessProducer::new(0xabab, 0x01, 0x23));

        assert_eq!(
            BcmChangeBrightnessProducer::try_deserialize(&data),
            Ok(producer)
        );
    }

    #[test]
    fn change_brightness_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x01];

        assert_eq!(
            BcmChangeBrightnessProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
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

    #[test]
    fn change_brightness_state_serialize_test() {
        let producer = BcmChangeBrightnessStateProducer::new(0xabab, 0x01, 0xffff_ffff);

        let expected_data = vec![0xab, 0xab, 0x01, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn change_brightness_state_deserialize_test() {
        let data = vec![0xab, 0xab, 0x01, 0xff, 0xff, 0xff, 0xff];

        let producer = Box::new(BcmChangeBrightnessStateProducer::new(
            0xabab,
            0x01,
            0xffff_ffff,
        ));

        assert_eq!(
            BcmChangeBrightnessStateProducer::try_deserialize(&data),
            Ok(producer)
        );
    }

    #[test]
    fn change_brightness_state_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x01, 0xff, 0xff, 0xff];

        assert_eq!(
            BcmChangeBrightnessStateProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
