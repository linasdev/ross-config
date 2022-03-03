extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use ross_protocol::convert_packet::ConvertPacket;
use ross_protocol::event::relay::{RelayDoubleExclusiveValue, RelaySetValueEvent, RelayValue};
use ross_protocol::packet::Packet;

use crate::producer::{Producer, ProducerError, RELAY_SET_VALUE_PRODUCER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct RelaySetValueProducer {
    relay_address: u16,
    index: u8,
    value: RelayValue,
}

impl RelaySetValueProducer {
    pub fn new(relay_address: u16, index: u8, value: RelayValue) -> Self {
        Self {
            relay_address,
            index,
            value,
        }
    }
}

impl Producer for RelaySetValueProducer {
    fn produce(
        &self,
        _value: ExtractorValue,
        _state_manager: &StateManager,
        device_address: u16,
    ) -> Result<Option<Packet>, ProducerError> {
        let event = RelaySetValueEvent {
            relay_address: self.relay_address,
            transmitter_address: device_address,
            index: self.index,
            value: self.value,
        };

        Ok(Some(event.to_packet()))
    }

    fn get_code(&self) -> u16 {
        RELAY_SET_VALUE_PRODUCER_CODE
    }
}

impl Serialize for RelaySetValueProducer {
    fn serialize(&self) -> Vec<u8> {
        let relay_address = self.relay_address.to_be_bytes();

        let mut data = vec![relay_address[0], relay_address[1], self.index];

        let mut value = self.value.serialize();

        data.append(&mut value);

        data
    }
}

impl TryDeserialize for RelaySetValueProducer {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 4 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let relay_address = u16::from_be_bytes(data[0..=1].try_into().unwrap());
        let index = data[2];
        let value = *RelayValue::try_deserialize(&data[3..])?;

        Ok(Box::new(Self {
            relay_address,
            index,
            value,
        }))
    }
}

impl Serialize for RelayValue {
    fn serialize(&self) -> Vec<u8> {
        match *self {
            RelayValue::Single(value) => vec![if value { 0x00 } else { 0x01 }],
            RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::FirstChannelOn) => vec![0x02],
            RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::SecondChannelOn) => vec![0x03],
            RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::NoChannelOn) => vec![0x04],
        }
    }
}

impl TryDeserialize for RelayValue {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 1 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => Ok(Box::new(RelayValue::Single(true))),
            0x01 => Ok(Box::new(RelayValue::Single(false))),
            0x02 => Ok(Box::new(RelayValue::DoubleExclusive(
                RelayDoubleExclusiveValue::FirstChannelOn,
            ))),
            0x03 => Ok(Box::new(RelayValue::DoubleExclusive(
                RelayDoubleExclusiveValue::SecondChannelOn,
            ))),
            0x04 => Ok(Box::new(RelayValue::DoubleExclusive(
                RelayDoubleExclusiveValue::NoChannelOn,
            ))),
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

    use ross_protocol::event::event_code::RELAY_SET_VALUE_EVENT_CODE;

    const PACKET: Packet = Packet {
        is_error: false,
        device_address: 0xabab,
        data: Vec::new(),
    };

    #[test]
    fn set_state_test() {
        let mut packet = PACKET;
        packet.data = vec![
            ((RELAY_SET_VALUE_EVENT_CODE >> 8) & 0xff) as u8, // event code
            ((RELAY_SET_VALUE_EVENT_CODE >> 0) & 0xff) as u8, // event code
            0x00,                                             // transmitter address
            0x00,                                             // transmitter address
            0x01,                                             // index
            0x03,                                             // value
        ];

        let state_manager = StateManager::new();

        let producer = RelaySetValueProducer::new(
            PACKET.device_address,
            0x01,
            RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::SecondChannelOn),
        );

        assert_eq!(
            producer.produce(ExtractorValue::None, &state_manager, 0x0000),
            Ok(Some(packet))
        );
    }

    #[test]
    fn set_state_serialize_test() {
        let producer = RelaySetValueProducer::new(
            0xabab,
            0x01,
            RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::SecondChannelOn),
        );

        let expected_data = vec![0xab, 0xab, 0x01, 0x03];

        assert_eq!(producer.serialize(), expected_data);
    }

    #[test]
    fn set_state_deserialize_test() {
        let data = vec![0xab, 0xab, 0x01, 0x03];

        let producer = RelaySetValueProducer::new(
            0xabab,
            0x01,
            RelayValue::DoubleExclusive(RelayDoubleExclusiveValue::SecondChannelOn),
        );

        assert_eq!(
            RelaySetValueProducer::try_deserialize(&data),
            Ok(Box::new(producer))
        );
    }

    #[test]
    fn set_state_deserialize_wrong_size_test() {
        let data = vec![0xab, 0xab, 0x01];

        assert_eq!(
            RelaySetValueProducer::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize)
        );
    }
}
