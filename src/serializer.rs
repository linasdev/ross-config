extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

use crate::config::Config;
use crate::creator::Creator;
use crate::event_processor::EventProcessor;
use crate::extractor::*;
use crate::filter::*;
use crate::matcher::Matcher;
use crate::producer::*;
use crate::Value;

#[macro_export]
macro_rules! try_deserialize_integer_from_vec {
    ($data:expr, $offset:expr, $integer_type:ty) => {{
        use core::convert::TryInto;
        use core::mem::size_of;

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

#[macro_export]
macro_rules! serialize_integer_to_vec {
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

        serialize_integer_to_vec!(data, config.initial_state.len(), u32);

        for state in config.initial_state.iter() {
            serialize_integer_to_vec!(data, *state.0, u32);
            let mut serialized_state = state.1.serialize();
            serialize_integer_to_vec!(data, serialized_state.len() as u8, u8);
            data.append(&mut serialized_state);
        }

        serialize_integer_to_vec!(data, config.event_processors.len(), u32);

        for event_processor in config.event_processors.iter() {
            let mut matcher = event_processor.matcher.serialize();
            serialize_integer_to_vec!(data, matcher.len() as u32, u32);
            data.append(&mut matcher);

            serialize_integer_to_vec!(data, event_processor.creators.len(), u32);

            for creator in event_processor.creators.iter() {
                serialize_integer_to_vec!(data, creator.extractor.get_code(), u16);
                let mut extractor = creator.extractor.serialize();
                serialize_integer_to_vec!(data, extractor.len() as u8, u8);
                data.append(&mut extractor);

                serialize_integer_to_vec!(data, creator.producer.get_code(), u16);
                let mut producer = creator.producer.serialize();
                serialize_integer_to_vec!(data, producer.len() as u8, u8);
                data.append(&mut producer);

                let matcher_exists = if creator.matcher.is_some() { 1 } else { 0 };
                serialize_integer_to_vec!(data, matcher_exists, u8);

                if let Some(matcher) = &creator.matcher {
                    let mut matcher = matcher.serialize();
                    serialize_integer_to_vec!(data, matcher.len() as u32, u32);
                    data.append(&mut matcher);
                }
            }
        }

        Ok(data)
    }

    pub fn deserialize(data: &[u8]) -> Result<Config, ConfigSerializerError> {
        let mut offset = 0;

        let initial_state_count = try_deserialize_integer_from_vec!(data, offset, u32);
        let mut initial_state = BTreeMap::new();

        for _ in 0..initial_state_count {
            let state_index = try_deserialize_integer_from_vec!(data, offset, u32);
            let serialized_state_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;

            let state_value =
                *Value::try_deserialize(&data[offset..offset + serialized_state_len])?;
            offset += serialized_state_len;

            initial_state.insert(state_index, state_value);
        }

        let event_processor_count = try_deserialize_integer_from_vec!(data, offset, u32);

        let mut event_processors = vec![];
        event_processors.reserve(event_processor_count as usize);

        for _ in 0..event_processor_count {
            let matcher_len = try_deserialize_integer_from_vec!(data, offset, u32) as usize;
            let matcher = *Matcher::try_deserialize(
                &data[offset..offset + matcher_len],
            )?;
            offset += matcher_len;

            let creator_count = try_deserialize_integer_from_vec!(data, offset, u32);

            let mut creators = vec![];
            creators.reserve(creator_count as usize);

            for _ in 0..creator_count {
                let extractor_code = try_deserialize_integer_from_vec!(data, offset, u16);
                let extractor_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
                let extractor = Self::try_deserialize_extractor_from_vec(
                    &data[offset..offset + extractor_len],
                    extractor_code,
                )?;
                offset += extractor_len;

                let producer_code = try_deserialize_integer_from_vec!(data, offset, u16);
                let producer_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
                let producer = Self::try_deserialize_producer_from_vec(
                    &data[offset..offset + producer_len],
                    producer_code,
                )?;
                offset += producer_len;

                let mut matcher = None;
                let matcher_exists = try_deserialize_integer_from_vec!(data, offset, u8) != 0;
                if matcher_exists {
                    let matcher_len = try_deserialize_integer_from_vec!(data, offset, u32) as usize;
                    matcher = Some(*Matcher::try_deserialize(
                        &data[offset..offset + matcher_len],
                    )?);
                    offset += matcher_len;
                }

                creators.push(Creator {
                    extractor,
                    producer,
                    matcher,
                });
            }

            event_processors.push(EventProcessor { matcher, creators });
        }

        Ok(Config {
            initial_state,
            event_processors,
        })
    }

    pub fn try_deserialize_extractor_from_vec(
        data: &[u8],
        extractor_code: u16,
    ) -> Result<Box<dyn Extractor>, ConfigSerializerError> {
        match extractor_code {
            NONE_EXTRACTOR_CODE => Ok(NoneExtractor::try_deserialize(data)?),
            PACKET_EXTRACTOR_CODE => Ok(PacketExtractor::try_deserialize(data)?),
            EVENT_CODE_EXTRACTOR_CODE => Ok(EventCodeExtractor::try_deserialize(data)?),
            EVENT_PRODUCER_ADDRESS_EXTRACTOR_CODE => {
                Ok(EventProducerAddressExtractor::try_deserialize(data)?)
            }
            MESSAGE_CODE_EXTRACTOR_CODE => Ok(MessageCodeExtractor::try_deserialize(data)?),
            MESSAGE_VALUE_EXTRACTOR_CODE => Ok(MessageValueExtractor::try_deserialize(data)?),
            BUTTON_INDEX_EXTRACTOR_CODE => Ok(ButtonIndexExtractor::try_deserialize(data)?),
            _ => Err(ConfigSerializerError::UnknownExtractor),
        }
    }

    pub fn try_deserialize_filter_from_vec(
        data: &[u8],
        filter_code: u16,
    ) -> Result<Box<dyn Filter>, ConfigSerializerError> {
        match filter_code {
            VALUE_EQUAL_TO_CONST_FILTER_CODE => Ok(ValueEqualToConstFilter::try_deserialize(data)?),
            STATE_EQUAL_TO_CONST_FILTER_CODE => Ok(StateEqualToConstFilter::try_deserialize(data)?),
            STATE_EQUAL_TO_VALUE_FILTER_CODE => Ok(StateEqualToValueFilter::try_deserialize(data)?),
            INCREMENT_STATE_BY_CONST_FILTER_CODE => {
                Ok(IncrementStateByConstFilter::try_deserialize(data)?)
            }
            INCREMENT_STATE_BY_VALUE_FILTER_CODE => {
                Ok(IncrementStateByValueFilter::try_deserialize(data)?)
            }
            DECREMENT_STATE_BY_CONST_FILTER_CODE => {
                Ok(DecrementStateByConstFilter::try_deserialize(data)?)
            }
            DECREMENT_STATE_BY_VALUE_FILTER_CODE => {
                Ok(DecrementStateByValueFilter::try_deserialize(data)?)
            }
            SET_STATE_TO_CONST_FILTER_CODE => Ok(SetStateToConstFilter::try_deserialize(data)?),
            SET_STATE_TO_VALUE_FILTER_CODE => Ok(SetStateToValueFilter::try_deserialize(data)?),
            FLIP_STATE_FILTER_CODE => Ok(FlipStateFilter::try_deserialize(data)?),
            _ => Err(ConfigSerializerError::UnknownExtractor),
        }
    }

    pub fn try_deserialize_producer_from_vec(
        data: &[u8],
        producer_code: u16,
    ) -> Result<Box<dyn Producer>, ConfigSerializerError> {
        match producer_code {
            NONE_PRODUCER_CODE => Ok(NoneProducer::try_deserialize(data)?),
            PACKET_PRODUCER_CODE => Ok(PacketProducer::try_deserialize(data)?),
            MESSAGE_PRODUCER_CODE => Ok(MessageProducer::try_deserialize(data)?),
            BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE => {
                Ok(BcmChangeBrightnessProducer::try_deserialize(data)?)
            }
            BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE => {
                Ok(BcmChangeBrightnessStateProducer::try_deserialize(data)?)
            }
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
            matcher: Matcher::Single {
                extractor: Box::new(EventCodeExtractor::new()),
                filter: Box::new(ValueEqualToConstFilter::new(Value::U8(0xff))),
            },
            creators: vec![Creator {
                extractor: Box::new(NoneExtractor::new()),
                producer: Box::new(BcmChangeBrightnessStateProducer::new(0xabab, 0xff, 0)),
                matcher: None,
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
            0x02, // state value len
            0x00, 0xff, // state value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x09, // matcher len
            0x00, // matcher enum code
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x02, // filter len
            0x00, 0xff, // value
            0x00, 0x00, 0x00, 0x01, // creator count
            0x00, 0x00, // NONE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x04, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
            0x07, // producer len
            0xab, 0xab, // bcm_address
            0xff, // channel
            0x00, 0x00, 0x00, 0x00, // state_index
            0x00, // matcher exists
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
            0x02, // state value len
            0x00, 0xff, // state value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x09, // matcher len
            0x00, // matcher enum code
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x02, // filter len
            0x00, 0xff, // value
            0x00, 0x00, 0x00, 0x01, // creator count
            0x00, 0x00, // NONE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x04, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
            0x07, // producer len
            0xab, 0xab, // bcm_address
            0xff, // channel
            0x00, 0x00, 0x00, 0x00, // state_index
            0x00, // matcher exists
        ];

        let config = ConfigSerializer::deserialize(&data).unwrap();

        assert_eq!(config.initial_state.len(), 1);
        assert_eq!(*config.initial_state.get(&0).unwrap(), Value::U8(0xff));
        assert_eq!(config.event_processors.len(), 1);
    }

    #[test]
    fn if_match_serialize_test() {
        let mut initial_state = BTreeMap::new();
        initial_state.insert(0, Value::U8(0xff));

        let mut event_processors = vec![];
        event_processors.push(EventProcessor {
            matcher: Matcher::Single {
                extractor: Box::new(EventCodeExtractor::new()),
                filter: Box::new(ValueEqualToConstFilter::new(Value::U8(0xff))),
            },
            creators: vec![Creator {
                extractor: Box::new(NoneExtractor::new()),
                producer: Box::new(BcmChangeBrightnessStateProducer::new(0xabab, 0xff, 0)),
                matcher: Some(Matcher::Single {
                    extractor: Box::new(EventCodeExtractor::new()),
                    filter: Box::new(ValueEqualToConstFilter::new(Value::U8(0xff))),
                }),
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
            0x02, // state_value len
            0x00, 0xff, // state_value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x09, // matcher len
            0x00, // matcher enum code
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x02, // filter len
            0x00, 0xff, // value
            0x00, 0x00, 0x00, 0x01, // creator count
            0x00, 0x00, // NONE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x04, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
            0x07, // producer len
            0xab, 0xab, // bcm_address
            0xff, // channel
            0x00, 0x00, 0x00, 0x00, // state_index
            0x01, // matcher exists
            0x00, 0x00, 0x00, 0x09, // matcher len
            0x00, // matcher enum code
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x02, // filter len
            0x00, 0xff, // value
        ];

        assert_eq!(data, expected_data);
    }

    #[test]
    fn if_match_deserialize_test() {
        let data = vec![
            0x00, 0x00, 0x00, 0x01, // initial state count
            0x00, 0x00, 0x00, 0x00, // state_index
            0x02, // state_value len
            0x00, 0xff, // state_value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x09, // matcher len
            0x00, // matcher enum code
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x02, // filter len
            0x00, 0xff, // value
            0x00, 0x00, 0x00, 0x01, // creator count
            0x00, 0x00, // NONE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x04, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
            0x07, // producer len
            0xab, 0xab, // bcm_address
            0xff, // channel
            0x00, 0x00, 0x00, 0x00, // state_index
            0x01, // matcher exists
            0x00, 0x00, 0x00, 0x09, // matcher len
            0x00, // matcher enum code
            0x00, 0x02, // EVENT_CODE_EXTRACTOR_CODE
            0x00, // extractor len
            0x00, 0x00, // VALUE_EQUAL_TO_CONST_FILTER_CODE
            0x02, // filter len
            0x00, 0xff, // value
        ];

        let config = ConfigSerializer::deserialize(&data).unwrap();

        assert_eq!(config.initial_state.len(), 1);
        assert_eq!(*config.initial_state.get(&0).unwrap(), Value::U8(0xff));
        assert_eq!(config.event_processors.len(), 1);
    }
}
