extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::bcm::BcmValue;
use ross_protocol::event::bcm::{BcmAnimateBrightnessEvent, BcmChangeBrightnessEvent};
use ross_protocol::packet::Packet;

use crate::producer::{
    Producer, ProducerError, BCM_ANIMATE_BRIGHTNESS_PRODUCER_CODE,
    BCM_ANIMATE_BRIGHTNESS_STATE_PRODUCER_CODE, BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE,
    BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE,
};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BcmChangeBrightnessProducer {
    bcm_address: u16,
    index: u8,
    value: BcmValue,
}

impl BcmChangeBrightnessProducer {
    pub fn new(bcm_address: u16, index: u8, value: BcmValue) -> Self {
        Self {
            bcm_address,
            index,
            value,
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
            index: self.index,
            value: self.value.clone(),
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

        let mut data = vec![bcm_address[0], bcm_address[1], self.index];

        let mut value = self.value.serialize();

        data.append(&mut value);

        return data;
    }
}

impl TryDeserialize for BcmChangeBrightnessProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 5 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let bcm_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let index = data[2];
        let value = *BcmValue::try_deserialize(&data[3..])?;

        Ok(Box::new(Self {
            bcm_address,
            index,
            value,
        }))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BcmChangeBrightnessStateProducer {
    bcm_address: u16,
    index: u8,
    state_index: u32,
}

impl BcmChangeBrightnessStateProducer {
    pub fn new(bcm_address: u16, index: u8, state_index: u32) -> Self {
        Self {
            bcm_address,
            index,
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
        let current_value = match state_manager.get_value(self.state_index) {
            Some(Value::U8(value)) => BcmValue::Single(*value),
            Some(Value::Rgb(r, g, b)) => BcmValue::Rgb(*r, *g, *b),
            Some(Value::Rgbw(r, g, b, w)) => BcmValue::Rgbw(*r, *g, *b, *w),
            _ => return Err(ProducerError::WrongStateType),
        };

        let event = BcmChangeBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_address,
            index: self.index,
            value: current_value,
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
            self.index,
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
        let index = data[2];
        let state_index = u32::from_be_bytes(data[3..=6].try_into().unwrap());

        Ok(Box::new(Self {
            bcm_address,
            index,
            state_index,
        }))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BcmAnimateBrightnessProducer {
    bcm_address: u16,
    index: u8,
    duration: u32,
    target_value: BcmValue,
}

impl BcmAnimateBrightnessProducer {
    pub fn new(bcm_address: u16, index: u8, duration: u32, target_value: BcmValue) -> Self {
        Self {
            bcm_address,
            index,
            duration,
            target_value,
        }
    }
}

impl Producer for BcmAnimateBrightnessProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let event = BcmAnimateBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_address,
            index: self.index,
            duration: self.duration,
            target_value: self.target_value.clone(),
        };

        Ok(Some(event.to_packet()))
    }

    fn get_code(&self) -> u16 {
        BCM_ANIMATE_BRIGHTNESS_PRODUCER_CODE
    }
}

impl Serialize for BcmAnimateBrightnessProducer {
    fn serialize(&self) -> Vec<u8> {
        let bcm_address = self.bcm_address.to_be_bytes();
        let duration = self.duration.to_be_bytes();

        let mut data = vec![
            bcm_address[0],
            bcm_address[1],
            self.index,
            duration[0],
            duration[1],
            duration[2],
            duration[3],
        ];

        let mut target_value = self.target_value.serialize();

        data.append(&mut target_value);

        return data;
    }
}

impl TryDeserialize for BcmAnimateBrightnessProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 9 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let bcm_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let index = data[2];
        let duration = u32::from_be_bytes(data[3..=6].try_into().unwrap());
        let target_value = *BcmValue::try_deserialize(&data[7..])?;

        Ok(Box::new(Self {
            bcm_address,
            index,
            duration,
            target_value,
        }))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct BcmAnimateBrightnessStateProducer {
    bcm_address: u16,
    index: u8,
    duration: u32,
    state_index: u32,
}

impl BcmAnimateBrightnessStateProducer {
    pub fn new(bcm_address: u16, index: u8, duration: u32, state_index: u32) -> Self {
        Self {
            bcm_address,
            index,
            duration,
            state_index,
        }
    }
}

impl Producer for BcmAnimateBrightnessStateProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let current_value = match state_manager.get_value(self.state_index) {
            Some(Value::U8(value)) => BcmValue::Single(*value),
            Some(Value::Rgb(r, g, b)) => BcmValue::Rgb(*r, *g, *b),
            Some(Value::Rgbw(r, g, b, w)) => BcmValue::Rgbw(*r, *g, *b, *w),
            _ => return Err(ProducerError::WrongStateType),
        };

        let event = BcmAnimateBrightnessEvent {
            bcm_address: self.bcm_address,
            transmitter_address: device_address,
            index: self.index,
            duration: self.duration,
            target_value: current_value,
        };

        Ok(Some(event.to_packet()))
    }

    fn get_code(&self) -> u16 {
        BCM_ANIMATE_BRIGHTNESS_STATE_PRODUCER_CODE
    }
}

impl Serialize for BcmAnimateBrightnessStateProducer {
    fn serialize(&self) -> Vec<u8> {
        let bcm_address = self.bcm_address.to_be_bytes();
        let duration = self.duration.to_be_bytes();
        let state_index = self.state_index.to_be_bytes();

        vec![
            bcm_address[0],
            bcm_address[1],
            self.index,
            duration[0],
            duration[1],
            duration[2],
            duration[3],
            state_index[0],
            state_index[1],
            state_index[2],
            state_index[3],
        ]
    }
}

impl TryDeserialize for BcmAnimateBrightnessStateProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 11 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let bcm_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let index = data[2];
        let duration = u32::from_be_bytes(data[3..=6].try_into().unwrap());
        let state_index = u32::from_be_bytes(data[7..=10].try_into().unwrap());

        Ok(Box::new(Self {
            bcm_address,
            index,
            duration,
            state_index,
        }))
    }
}

impl Serialize for BcmValue {
    fn serialize(&self) -> Vec<u8> {
        match *self {
            BcmValue::Binary(value) => vec![0x00, if value { 0x01 } else { 0x00 }],
            BcmValue::Single(value) => vec![0x01, value],
            BcmValue::Rgb(r, g, b) => vec![0x02, r, g, b],
            BcmValue::Rgbw(r, g, b, w) => vec![0x03, r, g, b, w],
        }
    }
}

impl TryDeserialize for BcmValue {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => {
                if data.len() < 2 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(BcmValue::Binary(data[1] != 0x00)))
            }
            0x01 => Ok(Box::new(BcmValue::Single(data[1]))),
            0x02 => {
                if data.len() < 4 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(BcmValue::Rgb(data[1], data[2], data[3])))
            }
            0x03 => {
                if data.len() < 5 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(BcmValue::Rgbw(data[1], data[2], data[3], data[4])))
            }
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    use ross_protocol::event::event_code::{
        BCM_ANIMATE_BRIGHTNESS_EVENT_CODE, BCM_CHANGE_BRIGHTNESS_EVENT_CODE,
    };

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
            0x01,                                                   // index
            0x02,                                                   // value
            0x23,                                                   // value
            0x45,                                                   // value
            0x67,                                                   // value
        ];

        let state_manager = StateManager::new();

        let producer = BcmChangeBrightnessProducer::new(
            PACKET.device_address,
            0x01,
            BcmValue::Rgb(0x23, 0x45, 0x67),
        );

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn change_brightness_serialize_test() {
        let producer =
            BcmChangeBrightnessProducer::new(0xabab, 0x01, BcmValue::Rgb(0x23, 0x45, 0x67));

        let expected_data = vec![0xab, 0xab, 0x01, 0x02, 0x23, 0x45, 0x67];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn change_brightness_deserialize_test() {
        let data = vec![0xab, 0xab, 0x01, 0x02, 0x23, 0x45, 0x67];

        let producer =
            BcmChangeBrightnessProducer::new(0xabab, 0x01, BcmValue::Rgb(0x23, 0x45, 0x67));

        assert_eq!(
            BcmChangeBrightnessProducer::try_deserialize(&data),
            Ok(Box::new(producer))
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
            0x01,                                                   // index
            0x02,                                                   // value
            0x23,                                                   // value
            0x45,                                                   // value
            0x67,                                                   // value
        ];

        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::Rgb(0x23, 0x45, 0x67));

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

    #[test]
    fn animate_brightness_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                    // transmitter address
            0x00,                                                    // transmitter address
            0x01,                                                    // index
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0x02,                                                    // target value
            0x23,                                                    // target value
            0x45,                                                    // target value
            0x67,                                                    // target value
        ];

        let state_manager = StateManager::new();

        let producer = BcmAnimateBrightnessProducer::new(
            PACKET.device_address,
            0x01,
            0xabab_abab,
            BcmValue::Rgb(0x23, 0x45, 0x67),
        );

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn animate_brightness_serialize_test() {
        let producer = BcmAnimateBrightnessProducer::new(
            0xabab,
            0x01,
            0xabab_abab,
            BcmValue::Rgb(0x23, 0x45, 0x67),
        );

        let expected_data = vec![
            0xab, 0xab, 0x01, 0xab, 0xab, 0xab, 0xab, 0x02, 0x23, 0x45, 0x67,
        ];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn animate_brightness_deserialize_test() {
        let data = vec![
            0xab, 0xab, 0x01, 0xab, 0xab, 0xab, 0xab, 0x02, 0x23, 0x45, 0x67,
        ];

        let producer = BcmAnimateBrightnessProducer::new(
            0xabab,
            0x01,
            0xabab_abab,
            BcmValue::Rgb(0x23, 0x45, 0x67),
        );

        assert_eq!(
            BcmAnimateBrightnessProducer::try_deserialize(&data),
            Ok(Box::new(producer))
        );
    }

    #[test]
    fn animate_brightness_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x01];

        assert_eq!(
            BcmAnimateBrightnessProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }

    #[test]
    fn animate_brightness_state_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((BCM_ANIMATE_BRIGHTNESS_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                                    // transmitter address
            0x00,                                                    // transmitter address
            0x01,                                                    // index
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0xab,                                                    // duration
            0x02,                                                    // target value
            0x23,                                                    // target value
            0x45,                                                    // target value
            0x67,                                                    // target value
        ];

        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::Rgb(0x23, 0x45, 0x67));

        let producer = BcmAnimateBrightnessStateProducer::new(0xabab, 0x01, 0xabab_abab, 0);

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn animate_brightness_state_serialize_test() {
        let producer =
            BcmAnimateBrightnessStateProducer::new(0xabab, 0x01, 0xabab_abab, 0xffff_ffff);

        let expected_data = vec![
            0xab, 0xab, 0x01, 0xab, 0xab, 0xab, 0xab, 0xff, 0xff, 0xff, 0xff,
        ];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn animate_brightness_state_deserialize_test() {
        let data = vec![
            0xab, 0xab, 0x01, 0xab, 0xab, 0xab, 0xab, 0xff, 0xff, 0xff, 0xff,
        ];

        let producer = Box::new(BcmAnimateBrightnessStateProducer::new(
            0xabab,
            0x01,
            0xabab_abab,
            0xffff_ffff,
        ));

        assert_eq!(
            BcmAnimateBrightnessStateProducer::try_deserialize(&data),
            Ok(producer)
        );
    }

    #[test]
    fn animate_brightness_state_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x02, 0xff, 0xff, 0xff];

        assert_eq!(
            BcmAnimateBrightnessStateProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }

    #[test]
    fn bcm_value_single_serialize_test() {
        let value = BcmValue::Single(0xab);

        let expected_data = vec![0x01, 0xab];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn bcm_value_single_deserialize_test() {
        let data = vec![0x01, 0xab];

        let expected_value = Box::new(BcmValue::Single(0xab));

        assert_eq!(BcmValue::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn bcm_value_rgb_serialize_test() {
        let value = BcmValue::Rgb(0xab, 0xab, 0xab);

        let expected_data = vec![0x02, 0xab, 0xab, 0xab];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn bcm_value_rgb_deserialize_test() {
        let data = vec![0x02, 0xab, 0xab, 0xab];

        let expected_value = Box::new(BcmValue::Rgb(0xab, 0xab, 0xab));

        assert_eq!(BcmValue::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn bcm_value_rgbw_serialize_test() {
        let value = BcmValue::Rgbw(0xab, 0xab, 0xab, 0xab);

        let expected_data = vec![0x03, 0xab, 0xab, 0xab, 0xab];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn bcm_value_rgbw_deserialize_test() {
        let data = vec![0x03, 0xab, 0xab, 0xab, 0xab];

        let expected_value = Box::new(BcmValue::Rgbw(0xab, 0xab, 0xab, 0xab));

        assert_eq!(BcmValue::try_deserialize(&data), Ok(expected_value));
    }
}
