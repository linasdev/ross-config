extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::relay::{RelayFlipStateEvent, RelaySetStateEvent};
use ross_protocol::packet::Packet;

use crate::producer::{
    Producer, ProducerError, RELAY_FLIP_STATE_PRODUCER_CODE, RELAY_SET_STATE_PRODUCER_CODE,
    RELAY_SET_STATE_STATE_PRODUCER_CODE,
};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::{ExtractorValue, Value};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct RelaySetStateProducer {
    relay_address: u16,
    index: u8,
    state: bool,
}

impl RelaySetStateProducer {
    pub fn new(relay_address: u16, index: u8, state: bool) -> Self {
        Self {
            relay_address,
            index,
            state,
        }
    }
}

impl Producer for RelaySetStateProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let event = RelaySetStateEvent {
            relay_address: self.relay_address,
            transmitter_address: device_address,
            index: self.index,
            state: self.state,
        };

        Ok(Some(event.to_packet()))
    }

    fn get_code(&self) -> u16 {
        RELAY_SET_STATE_PRODUCER_CODE
    }
}

impl Serialize for RelaySetStateProducer {
    fn serialize(&self) -> Vec<u8> {
        let relay_address = self.relay_address.to_be_bytes();

        vec![
            relay_address[0],
            relay_address[1],
            self.index,
            if self.state { 0x01 } else { 0x00 },
        ]
    }
}

impl TryDeserialize for RelaySetStateProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() != 4 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let relay_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let index = data[2];
        let state = data[3] != 0x00;

        Ok(Box::new(Self {
            relay_address,
            index,
            state,
        }))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct RelaySetStateStateProducer {
    relay_address: u16,
    index: u8,
    state_index: u32,
}

impl RelaySetStateStateProducer {
    pub fn new(relay_address: u16, index: u8, state_index: u32) -> Self {
        Self {
            relay_address,
            index,
            state_index,
        }
    }
}

impl Producer for RelaySetStateStateProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let current_value = match state_manager.get_value(self.state_index) {
            Some(Value::Bool(value)) => *value,
            _ => return Err(ProducerError::WrongStateType),
        };

        let event = RelaySetStateEvent {
            relay_address: self.relay_address,
            transmitter_address: device_address,
            index: self.index,
            state: current_value,
        };

        Ok(Some(event.to_packet()))
    }

    fn get_code(&self) -> u16 {
        RELAY_SET_STATE_STATE_PRODUCER_CODE
    }
}

impl Serialize for RelaySetStateStateProducer {
    fn serialize(&self) -> Vec<u8> {
        let relay_address = self.relay_address.to_be_bytes();
        let state_index = self.state_index.to_be_bytes();

        vec![
            relay_address[0],
            relay_address[1],
            self.index,
            state_index[0],
            state_index[1],
            state_index[2],
            state_index[3],
        ]
    }
}

impl TryDeserialize for RelaySetStateStateProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() != 7 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let relay_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let index = data[2];
        let state_index = u32::from_be_bytes(data[3..=6].try_into().unwrap());

        Ok(Box::new(Self {
            relay_address,
            index,
            state_index,
        }))
    }
}

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct RelayFlipStateProducer {
    relay_address: u16,
    index: u8,
}

impl RelayFlipStateProducer {
    pub fn new(relay_address: u16, index: u8) -> Self {
        Self {
            relay_address,
            index,
        }
    }
}

impl Producer for RelayFlipStateProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let event = RelayFlipStateEvent {
            relay_address: self.relay_address,
            transmitter_address: device_address,
            index: self.index,
        };

        Ok(Some(event.to_packet()))
    }

    fn get_code(&self) -> u16 {
        RELAY_FLIP_STATE_PRODUCER_CODE
    }
}

impl Serialize for RelayFlipStateProducer {
    fn serialize(&self) -> Vec<u8> {
        let relay_address = self.relay_address.to_be_bytes();

        vec![relay_address[0], relay_address[1], self.index]
    }
}

impl TryDeserialize for RelayFlipStateProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() != 3 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let relay_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let index = data[2];

        Ok(Box::new(Self {
            relay_address,
            index,
        }))
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;

    use alloc::vec;
    use alloc::vec::Vec;

    use ross_protocol::event::event_code::{
        RELAY_FLIP_STATE_EVENT_CODE, RELAY_SET_STATE_EVENT_CODE,
    };

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: Vec::new(),
    };

    #[test]
    fn set_state_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((RELAY_SET_STATE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_SET_STATE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                             // transmitter address
            0x00,                                             // transmitter address
            0x01,                                             // index
            0x01,                                             // state
        ];

        let state_manager = StateManager::new();

        let producer = RelaySetStateProducer::new(PACKET.device_address, 0x01, true);

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn set_state_serialize_test() {
        let producer = RelaySetStateProducer::new(0xabab, 0x01, true);

        let expected_data = vec![0xab, 0xab, 0x01, 0x01];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn set_state_deserialize_test() {
        let data = vec![0xab, 0xab, 0x01, 0x01];

        let producer = RelaySetStateProducer::new(0xabab, 0x01, true);

        assert_eq!(
            RelaySetStateProducer::try_deserialize(&data),
            Ok(Box::new(producer))
        );
    }

    #[test]
    fn set_state_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x01];

        assert_eq!(
            RelaySetStateProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }

    #[test]
    fn set_state_state_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((RELAY_SET_STATE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_SET_STATE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                             // transmitter address
            0x00,                                             // transmitter address
            0x01,                                             // index
            0x01,                                             // state
        ];

        let mut state_manager = StateManager::new();
        state_manager.set_value(0, Value::Bool(true));

        let producer = RelaySetStateStateProducer::new(0xabab, 0x01, 0);

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn set_state_state_serialize_test() {
        let producer = RelaySetStateStateProducer::new(0xabab, 0x01, 0xffff_ffff);

        let expected_data = vec![0xab, 0xab, 0x01, 0xff, 0xff, 0xff, 0xff];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn set_state_state_deserialize_test() {
        let data = vec![0xab, 0xab, 0x01, 0xff, 0xff, 0xff, 0xff];

        let producer = Box::new(RelaySetStateStateProducer::new(0xabab, 0x01, 0xffff_ffff));

        assert_eq!(
            RelaySetStateStateProducer::try_deserialize(&data),
            Ok(producer)
        );
    }

    #[test]
    fn set_state_state_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x01, 0xff, 0xff, 0xff];

        assert_eq!(
            RelaySetStateStateProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }

    #[test]
    fn flip_state_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((RELAY_FLIP_STATE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_FLIP_STATE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                              // transmitter address
            0x00,                                              // transmitter address
            0x01,                                              // index
        ];

        let state_manager = StateManager::new();

        let producer = RelayFlipStateProducer::new(PACKET.device_address, 0x01);

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn flip_state_serialize_test() {
        let producer = RelayFlipStateProducer::new(0xabab, 0x01);

        let expected_data = vec![0xab, 0xab, 0x01];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn flip_state_deserialize_test() {
        let data = vec![0xab, 0xab, 0x01];

        let producer = RelayFlipStateProducer::new(0xabab, 0x01);

        assert_eq!(
            RelayFlipStateProducer::try_deserialize(&data),
            Ok(Box::new(producer))
        );
    }

    #[test]
    fn flip_state_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab];

        assert_eq!(
            RelayFlipStateProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
