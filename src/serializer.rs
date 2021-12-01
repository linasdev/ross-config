extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;
use core::mem::size_of;

use crate::config::Config;
use crate::creator::Creator;
use crate::event_processor::EventProcessor;
use crate::extractor::*;
use crate::filter::*;
use crate::matcher::Matcher;
use crate::producer::*;
use crate::Value;

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
            write_integer_to_vec!(data, serialized_state.len() as u8, u8);
            data.append(&mut serialized_state);
        }

        write_integer_to_vec!(data, config.event_processors.len(), u32);

        for event_processor in config.event_processors.iter() {
            write_integer_to_vec!(data, event_processor.matchers.len(), u32);

            for matcher in event_processor.matchers.iter() {
                write_integer_to_vec!(data, matcher.extractor.get_code(), u16);
                let mut extractor = matcher.extractor.serialize();
                write_integer_to_vec!(data, extractor.len() as u8, u8);
                data.append(&mut extractor);

                write_integer_to_vec!(data, matcher.filter.get_code(), u16);
                let mut filter = matcher.filter.serialize();
                write_integer_to_vec!(data, filter.len() as u8, u8);
                data.append(&mut filter);
            }

            write_integer_to_vec!(data, event_processor.creators.len(), u32);

            for creator in event_processor.creators.iter() {
                write_integer_to_vec!(data, creator.extractor.get_code(), u16);
                let mut extractor = creator.extractor.serialize();
                write_integer_to_vec!(data, extractor.len() as u8, u8);
                data.append(&mut extractor);

                write_integer_to_vec!(data, creator.producer.get_code(), u16);
                let mut producer = creator.producer.serialize();
                write_integer_to_vec!(data, producer.len() as u8, u8);
                data.append(&mut producer);

                let matcher_count = if creator.matcher.is_some() { 1 } else { 0 };
                write_integer_to_vec!(data, matcher_count, u8);

                if let Some(matcher) = &creator.matcher {
                    write_integer_to_vec!(data, matcher.extractor.get_code(), u16);
                    let mut extractor = matcher.extractor.serialize();
                    write_integer_to_vec!(data, extractor.len() as u8, u8);
                    data.append(&mut extractor);

                    write_integer_to_vec!(data, matcher.filter.get_code(), u16);
                    let mut filter = matcher.filter.serialize();
                    write_integer_to_vec!(data, filter.len() as u8, u8);
                    data.append(&mut filter);
                }
            }
        }

        Ok(data)
    }

    pub fn deserialize(data: &[u8]) -> Result<Config, ConfigSerializerError> {
        let mut offset = 0;

        let initial_state_count = read_integer_from_vec!(data, offset, u32);
        let mut initial_state = BTreeMap::new();

        for _ in 0..initial_state_count {
            let state_index = read_integer_from_vec!(data, offset, u32);
            let serialized_state_len = read_integer_from_vec!(data, offset, u8) as usize;

            let state_value =
                *Value::try_deserialize(&data[offset..offset + serialized_state_len])?;
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
                let extractor_len = read_integer_from_vec!(data, offset, u8) as usize;
                let extractor = Self::read_extractor_from_vec(
                    &data[offset..offset + extractor_len],
                    extractor_code,
                )?;
                offset += extractor_len;

                let filter_code = read_integer_from_vec!(data, offset, u16);
                let filter_len = read_integer_from_vec!(data, offset, u8) as usize;
                let filter =
                    Self::read_filter_from_vec(&data[offset..offset + filter_len], filter_code)?;
                offset += filter_len;

                matchers.push(Matcher { extractor, filter });
            }

            let creator_count = read_integer_from_vec!(data, offset, u32);

            let mut creators = vec![];
            creators.reserve(creator_count as usize);

            for _ in 0..creator_count {
                let extractor_code = read_integer_from_vec!(data, offset, u16);
                let extractor_len = read_integer_from_vec!(data, offset, u8) as usize;
                let extractor = Self::read_extractor_from_vec(
                    &data[offset..offset + extractor_len],
                    extractor_code,
                )?;
                offset += extractor_len;

                let producer_code = read_integer_from_vec!(data, offset, u16);
                let producer_len = read_integer_from_vec!(data, offset, u8) as usize;
                let producer = Self::read_producer_from_vec(
                    &data[offset..offset + producer_len],
                    producer_code,
                )?;
                offset += producer_len;

                let mut matcher = None;
                let matcher_count = read_integer_from_vec!(data, offset, u8);
                if matcher_count != 0 {
                    let extractor_code = read_integer_from_vec!(data, offset, u16);
                    let extractor_len = read_integer_from_vec!(data, offset, u8) as usize;
                    let extractor = Self::read_extractor_from_vec(
                        &data[offset..offset + extractor_len],
                        extractor_code,
                    )?;
                    offset += extractor_len;

                    let filter_code = read_integer_from_vec!(data, offset, u16);
                    let filter_len = read_integer_from_vec!(data, offset, u8) as usize;
                    let filter = Self::read_filter_from_vec(
                        &data[offset..offset + filter_len],
                        filter_code,
                    )?;
                    offset += filter_len;

                    matcher = Some(Matcher { extractor, filter });
                }

                creators.push(Creator {
                    extractor,
                    producer,
                    matcher,
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

    fn read_filter_from_vec(
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

    fn read_producer_from_vec(
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
            matchers: vec![Matcher {
                extractor: Box::new(EventCodeExtractor::new()),
                filter: Box::new(ValueEqualToConstFilter::new(Value::U8(0xff))),
            }],
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
            0x02, // state_value len
            0x00, 0xff, // state_value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x01, // matcher count
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
            0x00, // matcher count
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
            0x02, // state_value len
            0x00, 0xff, // state_value
            0x00, 0x00, 0x00, 0x01, // event processor count
            0x00, 0x00, 0x00, 0x01, // matcher count
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
            0x00, // matcher count
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
            matchers: vec![Matcher {
                extractor: Box::new(EventCodeExtractor::new()),
                filter: Box::new(ValueEqualToConstFilter::new(Value::U8(0xff))),
            }],
            creators: vec![Creator {
                extractor: Box::new(NoneExtractor::new()),
                producer: Box::new(BcmChangeBrightnessStateProducer::new(0xabab, 0xff, 0)),
                matcher: Some(Matcher {
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
            0x00, 0x00, 0x00, 0x01, // matcher count
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
            0x01, // matcher count
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
            0x00, 0x00, 0x00, 0x01, // matcher count
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
            0x01, // matcher count
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
