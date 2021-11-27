extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::mem::{size_of, transmute_copy};

use crate::config::Config;
use crate::creator::Creator;
use crate::event_processor::EventProcessor;
use crate::extractor::*;
use crate::filter::*;
use crate::matcher::Matcher;
use crate::producer::*;
use crate::Value;

macro_rules! impl_item_read {
    ($item_code:expr, $item_type:ty, $data:expr, $offset:expr, $provided_code:expr) => {
        if $item_code == $provided_code {
            unsafe {
                const SIZE: usize = size_of::<$item_type>();
                let item = Box::new(transmute_copy::<[u8; SIZE], $item_type>(
                    $data[*$offset..*$offset + SIZE].try_into().unwrap(),
                ));
                *$offset += SIZE;

                return Ok(item);
            }
        }
    };
}

macro_rules! impl_item_write {
    ($item_code:expr, $item_type:ty, $data:expr, $item:expr) => {
        if let Some(item) = $item.downcast_ref::<$item_type>() {
            write_integer_to_vec!($data, $item_code, u16);

            unsafe {
                for byte in transmute_copy::<$item_type, [u8; size_of::<$item_type>()]>(item).iter()
                {
                    $data.push(*byte);
                }
            }

            return Ok(());
        }
    };
}

macro_rules! read_integer_from_vec {
    ($data:expr, $offset:expr, $integer_type:ty) => {{
        const SIZE: usize = size_of::<$integer_type>();

        if $data.len() < $offset + SIZE {
            return Err(ConfigSerializerError::WrongSize);
        }

        let integer =
            <$integer_type>::from_be_bytes($data[$offset..$offset + SIZE].try_into().unwrap());

        $offset += SIZE;

        integer
    }};
}

macro_rules! write_integer_to_vec {
    ($data:expr, $integer:expr, $integer_type:ty) => {
        for byte in <$integer_type>::to_be_bytes($integer as $integer_type).iter() {
            $data.push(*byte);
        }
    };
}

pub trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

pub trait TryDeserialize {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError>;
}

#[derive(Debug, PartialEq)]
pub enum ConfigSerializerError {
    WrongSize,
    UnknownEnumVariant,
    UnknownExtractor,
    UnknownFilter,
    UnknownProducer,
}

pub struct ConfigSerializer {}

impl ConfigSerializer {
    pub fn serialize(config: &Config) -> Result<Vec<u8>, ConfigSerializerError> {
        let mut data = vec![];

        write_integer_to_vec!(data, config.initial_state.len(), u32);

        for state in config.initial_state.iter() {
            write_integer_to_vec!(data, *state.0, u32);
            let mut serialized_state = state.1.serialize();
            write_integer_to_vec!(data, serialized_state.len() as u32, u32);
            data.append(&mut serialized_state);
        }

        write_integer_to_vec!(data, config.event_processors.len(), u32);

        for event_processor in config.event_processors.iter() {
            write_integer_to_vec!(data, event_processor.matchers.len(), u32);

            for matcher in event_processor.matchers.iter() {
                write_integer_to_vec!(data, matcher.extractor.get_code(), u32);
                let mut serialized_extractor = matcher.extractor.serialize();
                write_integer_to_vec!(data, serialized_extractor.len() as u32, u32);
                data.append(&mut serialized_extractor);
                
                Self::write_filter_to_vec(&mut data, &matcher.filter)?;
            }

            write_integer_to_vec!(data, event_processor.creators.len(), u32);

            for creator in event_processor.creators.iter() {
                write_integer_to_vec!(data, creator.extractor.get_code(), u32);
                let mut extractor = creator.extractor.serialize();
                write_integer_to_vec!(data, extractor.len() as u32, u32);
                data.append(&mut extractor);

                write_integer_to_vec!(data, creator.producer.get_code(), u32);
                let mut producer = creator.producer.serialize();
                write_integer_to_vec!(data, producer.len() as u32, u32);
                data.append(&mut producer);
            }
        }

        Ok(data)
    }

    pub fn deserialize(data: &Vec<u8>) -> Result<Config, ConfigSerializerError> {
        let mut offset = 0;

        let initial_state_count = read_integer_from_vec!(data, offset, u32);
        let mut initial_state = BTreeMap::new();

        for _ in 0..initial_state_count {
            let state_index = read_integer_from_vec!(data, offset, u32);
            let serialized_state_len = read_integer_from_vec!(data, offset, u32) as usize;

            let state_value = *Value::try_deserialize(&data[offset..offset + serialized_state_len])?;
            offset += serialized_state_len;

            initial_state.insert(state_index, state_value);
        }

        let event_processor_count = read_integer_from_vec!(data, offset, u32);

        let mut event_processors = vec![];
        event_processors.reserve(event_processor_count as usize);

        for _ in 0..event_processor_count {
            let matcher_count = read_integer_from_vec!(data, offset, u32);

            let mut matchers = vec![];
            matchers.reserve(matcher_count as usize);

            for _ in 0..matcher_count {
                let extractor_code = read_integer_from_vec!(data, offset, u16);
                let extractor_len = read_integer_from_vec!(data, offset, u32);
                let extractor = Self::read_extractor_from_vec(data, extractor_code)?;
                offset += extractor_len as usize;

                let filter_code = read_integer_from_vec!(data, offset, u16);
                let filter = Self::read_filter_from_vec(data, &mut offset, filter_code)?;

                matchers.push(Matcher { extractor, filter });
            }

            let creator_count = read_integer_from_vec!(data, offset, u32);

            let mut creators = vec![];
            creators.reserve(creator_count as usize);

            for _ in 0..creator_count {
                let extractor_code = read_integer_from_vec!(data, offset, u16);
                let extractor_len = read_integer_from_vec!(data, offset, u32);
                let extractor = Self::read_extractor_from_vec(data, extractor_code)?;
                offset += extractor_len as usize;

                let producer_code = read_integer_from_vec!(data, offset, u16);
                let producer_len = read_integer_from_vec!(data, offset, u32);
                let producer = Self::read_producer_from_vec(data, producer_code)?;
                offset += producer_len as usize;

                creators.push(Creator {
                    extractor,
                    producer,
                });
            }

            event_processors.push(EventProcessor { matchers, creators });
        }

        Ok(Config {
            initial_state,
            event_processors,
        })
    }

    fn read_extractor_from_vec(
        data: &Vec<u8>,
        extractor_code: u16,
    ) -> Result<Box<dyn Extractor>, ConfigSerializerError> {
        match extractor_code {
            NONE_EXTRACTOR_CODE => Ok(NoneExtractor::try_deserialize(data)?),
            PACKET_EXTRACTOR_CODE => Ok(PacketExtractor::try_deserialize(data)?),
            EVENT_CODE_EXTRACTOR_CODE => Ok(EventCodeExtractor::try_deserialize(data)?),
            EVENT_PRODUCER_ADDRESS_EXTRACTOR_CODE => Ok(EventProducerAddressExtractor::try_deserialize(data)?),
            MESSAGE_CODE_EXTRACTOR_CODE => Ok(MessageCodeExtractor::try_deserialize(data)?),
            MESSAGE_VALUE_EXTRACTOR_CODE => Ok(MessageValueExtractor::try_deserialize(data)?),
            BUTTON_INDEX_EXTRACTOR_CODE => Ok(ButtonIndexExtractor::try_deserialize(data)?),
            _ => Err(ConfigSerializerError::UnknownExtractor),
        }
    }

    fn read_filter_from_vec(
        data: &Vec<u8>,
        offset: &mut usize,
        filter_code: u16,
    ) -> Result<Box<dyn Filter>, ConfigSerializerError> {
        impl_item_read!(
            VALUE_EQUAL_TO_CONST_FILTER_CODE,
            ValueEqualToConstFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            STATE_EQUAL_TO_CONST_FILTER_CODE,
            StateEqualToConstFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            STATE_EQUAL_TO_VALUE_FILTER_CODE,
            StateEqualToValueFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            INCREMENT_STATE_BY_CONST_FILTER_CODE,
            IncrementStateByConstFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            INCREMENT_STATE_BY_VALUE_FILTER_CODE,
            IncrementStateByValueFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            DECREMENT_STATE_BY_CONST_FILTER_CODE,
            DecrementStateByConstFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            DECREMENT_STATE_BY_VALUE_FILTER_CODE,
            DecrementStateByValueFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            SET_STATE_TO_CONST_FILTER_CODE,
            SetStateToConstFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            SET_STATE_TO_VALUE_FILTER_CODE,
            SetStateToValueFilter,
            data,
            offset,
            filter_code
        );
        impl_item_read!(
            FLIP_STATE_FILTER_CODE,
            FlipStateFilter,
            data,
            offset,
            filter_code
        );
        Err(ConfigSerializerError::UnknownFilter)
    }

    fn write_filter_to_vec(
        data: &mut Vec<u8>,
        filter: &Box<dyn Filter>,
    ) -> Result<(), ConfigSerializerError> {
        impl_item_write!(
            VALUE_EQUAL_TO_CONST_FILTER_CODE,
            ValueEqualToConstFilter,
            data,
            filter
        );
        impl_item_write!(
            STATE_EQUAL_TO_CONST_FILTER_CODE,
            StateEqualToConstFilter,
            data,
            filter
        );
        impl_item_write!(
            STATE_EQUAL_TO_VALUE_FILTER_CODE,
            StateEqualToValueFilter,
            data,
            filter
        );
        impl_item_write!(
            INCREMENT_STATE_BY_CONST_FILTER_CODE,
            IncrementStateByConstFilter,
            data,
            filter
        );
        impl_item_write!(
            INCREMENT_STATE_BY_VALUE_FILTER_CODE,
            IncrementStateByValueFilter,
            data,
            filter
        );
        impl_item_write!(
            DECREMENT_STATE_BY_CONST_FILTER_CODE,
            DecrementStateByConstFilter,
            data,
            filter
        );
        impl_item_write!(
            DECREMENT_STATE_BY_VALUE_FILTER_CODE,
            DecrementStateByValueFilter,
            data,
            filter
        );
        impl_item_write!(
            SET_STATE_TO_CONST_FILTER_CODE,
            SetStateToConstFilter,
            data,
            filter
        );
        impl_item_write!(
            SET_STATE_TO_VALUE_FILTER_CODE,
            SetStateToValueFilter,
            data,
            filter
        );
        impl_item_write!(FLIP_STATE_FILTER_CODE, FlipStateFilter, data, filter);
        Err(ConfigSerializerError::UnknownFilter)
    }

    fn read_producer_from_vec(
        data: &Vec<u8>,
        producer_code: u16,
    ) -> Result<Box<dyn Producer>, ConfigSerializerError> {
        match producer_code {
            NONE_PRODUCER_CODE => Ok(NoneProducer::try_deserialize(data)?),
            PACKET_PRODUCER_CODE => Ok(PacketProducer::try_deserialize(data)?),
            MESSAGE_PRODUCER_CODE => Ok(MessageProducer::try_deserialize(data)?),
            BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE => Ok(BcmChangeBrightnessProducer::try_deserialize(data)?),
            BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE => Ok(BcmChangeBrightnessStateProducer::try_deserialize(data)?),
            _ => Err(ConfigSerializerError::UnknownProducer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_test() {
        let mut initial_state = BTreeMap::new();
        initial_state.insert(0, Value::U8(0xff));

        let mut event_processors = vec![];
        event_processors.push(EventProcessor {
            matchers: vec![Matcher {
                extractor: Box::new(EventCodeExtractor::new()),
                filter: Box::new(ValueEqualToConstFilter::new(Value::U8(0xff))),
            }],
            creators: vec![Creator {
                extractor: Box::new(NoneExtractor::new()),
                producer: Box::new(BcmChangeBrightnessStateProducer::new(0xff, 0xff, 0)),
            }],
        });

        let config = Config {
            initial_state,
            event_processors,
        };

        let data = ConfigSerializer::serialize(&config).unwrap();

        let expected_data = vec![
            0x00, 0x00, 0x00, 0x01, // initial state count
            0x00, 0x00, 0x00, 0x00, // state_index
            0x00, 0x00, 0x00, 0x02, // state_value len
            0x00, 0xff, // state_value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x01, // matcher count
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value
            0x00, 0x00, 0x00, 0x01, // creator count
            0x00, 0x00, // NONE_EXTRACTOR_CODE
            0x00, 0x04, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
            0xff, 0x00, // bcm_address
            0xff, 0x00, // channel
            0x00, 0x00, 0x00, 0x00, // state_index
        ];

        assert_eq!(data, expected_data);
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![];

        let err = ConfigSerializer::deserialize(&data).unwrap_err();

        assert_eq!(ConfigSerializerError::WrongSize, err);
    }

    #[test]
    fn deserialize_empty_test() {
        let data = vec![
            0x00, 0x00, 0x00, 0x00, // initial state count
            0x00, 0x00, 0x00, 0x00, // event processor count
        ];

        let config = ConfigSerializer::deserialize(&data).unwrap();

        assert_eq!(config.initial_state.len(), 0);
        assert_eq!(config.event_processors.len(), 0);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![
            0x00, 0x00, 0x00, 0x01, // initial state count
            0x00, 0x00, 0x00, 0x00, // state_index
            0x00, 0x00, 0x00, 0x02, // state_value len
            0x00, 0xff, // state_value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x01, // matcher count
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value
            0x00, 0x00, 0x00, 0x01, // creator count
            0x00, 0x00, // NONE_EXTRACTOR_CODE
            0x00, 0x04, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
            0xff, 0x00, // bcm_address
            0xff, 0x00, // channel
            0x00, 0x00, 0x00, 0x00, // state_index
        ];

        let config = ConfigSerializer::deserialize(&data).unwrap();

        assert_eq!(config.initial_state.len(), 1);
        assert_eq!(*config.initial_state.get(&0).unwrap(), Value::U8(0xff));
        assert_eq!(config.event_processors.len(), 1);
    }
}
