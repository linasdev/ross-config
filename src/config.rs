extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use core::convert::TryInto;
use core::mem::{size_of, transmute_copy};

use crate::Value;
use crate::event_processor::EventProcessor;
use crate::matcher::Matcher;
use crate::extractor::*;
use crate::filter::*;
use crate::filter::state_filter::*;
use crate::producer::*;
use crate::producer::state_producer::*;

macro_rules! impl_item_read {
    ($item_code:expr, $item_type:ty, $data:expr, $offset:expr, $provided_code:expr) => {
        if $item_code == $provided_code {
            unsafe {
                const SIZE: usize = size_of::<$item_type>();
                let item = Box::new(transmute_copy::<[u8; SIZE], $item_type>($data[*$offset..*$offset + SIZE].try_into().unwrap()));
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
                for byte in transmute_copy::<$item_type, [u8; size_of::<$item_type>()]>(item).iter() {
                    $data.push(*byte);
                }
            }

            return Ok(());
        }
    };
}

macro_rules! read_integer_from_vec {
    ($data:expr, $offset:expr, $integer_type:ty) => {
        {
            const SIZE: usize = size_of::<$integer_type>();

            if $data.len() < $offset + SIZE {
                return Err(ConfigSerializerError::WrongSize);
            }
    
            let integer = <$integer_type>::from_be_bytes($data[$offset..$offset + SIZE].try_into().unwrap());
    
            $offset += SIZE;
    
            integer
        }
    };
}

macro_rules! write_integer_to_vec {
    ($data:expr, $integer:expr, $integer_type:ty) => {
        for byte in <$integer_type>::to_be_bytes($integer as $integer_type).iter() {
            $data.push(*byte);
        }
    };
}

#[derive(Debug)]
pub struct Config {
    pub initial_state: BTreeMap<u32, Value>,
    pub event_processors: Vec<EventProcessor>,
}

#[derive(Debug, PartialEq)]
pub enum ConfigSerializerError {
    WrongSize,
    UnknownExtractor,
    UnknownFilter,
    UnknownProducer,
}

pub struct ConfigSerializer {}

impl ConfigSerializer {
    pub fn serialize(config: &Config) -> Result<Vec<u8>, ConfigSerializerError> {
        let mut data = vec!();

        write_integer_to_vec!(data, config.initial_state.len(), u32);

        for state in config.initial_state.iter() {
            write_integer_to_vec!(data, *state.0, u32);

            unsafe {
                for byte in transmute_copy::<Value, [u8; size_of::<Value>()]>(state.1).iter() {
                    data.push(*byte);
                }
            }
        }

        write_integer_to_vec!(data, config.event_processors.len(), u32);
    
        for event_processor in config.event_processors.iter() {
            write_integer_to_vec!(data, event_processor.matchers.len(), u32);
    
            for matcher in event_processor.matchers.iter() {
                Self::write_extractor_to_vec(&mut data, &matcher.extractor)?;
                Self::write_filter_to_vec(&mut data, &matcher.filter)?;
            }
            
            Self::write_extractor_to_vec(&mut data, &event_processor.extractor)?;
            Self::write_producer_to_vec(&mut data, &event_processor.producer)?;
        }
    
        Ok(data)
    }

    pub fn deserialize(data: &Vec<u8>) -> Result<Config, ConfigSerializerError> {
        let mut offset = 0;

        let initial_state_count = read_integer_from_vec!(data, offset, u32);
        let mut initial_state = BTreeMap::new();

        for _ in 0..initial_state_count {
            let state_index = read_integer_from_vec!(data, offset, u32);
            let state_value = unsafe {
                const SIZE: usize = size_of::<Value>();
                let item = transmute_copy::<[u8; SIZE], Value>(data[offset..offset + SIZE].try_into().unwrap());
                offset += SIZE;
                item
            };

            initial_state.insert(state_index, state_value);
        }

        let event_processor_count = read_integer_from_vec!(data, offset, u32);

        let mut event_processors = vec!();
        event_processors.reserve(event_processor_count as usize);
    
        for _ in 0..event_processor_count {
            let matcher_count = read_integer_from_vec!(data, offset, u32);
    
            let mut matchers = vec!();
            matchers.reserve(matcher_count as usize);
    
            for _ in 0..matcher_count {
                let extractor_code = read_integer_from_vec!(data, offset, u16);
                let extractor = Self::read_extractor_from_vec(data, &mut offset, extractor_code)?;
    
                let filter_code = read_integer_from_vec!(data, offset, u16);
                let filter = Self::read_filter_from_vec(data, &mut offset, filter_code)?;
    
                matchers.push(Matcher {
                    extractor,
                    filter,
                });
            }
    
            let extractor_code = read_integer_from_vec!(data, offset, u16);
            let extractor = Self::read_extractor_from_vec(data, &mut offset, extractor_code)?;
    
            let producer_code = read_integer_from_vec!(data, offset, u16);
            let producer = Self::read_producer_from_vec(data, &mut offset, producer_code)?;
    
            event_processors.push(EventProcessor {
                matchers,
                extractor,
                producer,
            });
        }
    
        Ok(Config {
            initial_state,
            event_processors,
        })
    }

    fn read_extractor_from_vec(data: &Vec<u8>, offset: &mut usize, extractor_code: u16) -> Result<Box<dyn Extractor>, ConfigSerializerError> {
        impl_item_read!(NONE_EXTRACTOR_CODE, NoneExtractor, data, offset, extractor_code);
        impl_item_read!(EVENT_CODE_EXTRACTOR_CODE, EventCodeExtractor, data, offset, extractor_code);
        Err(ConfigSerializerError::UnknownExtractor)
    }

    pub fn write_extractor_to_vec(data: &mut Vec<u8>, extractor: &Box<dyn Extractor>) -> Result<(), ConfigSerializerError> {
        impl_item_write!(NONE_EXTRACTOR_CODE, NoneExtractor, data, extractor);
        impl_item_write!(EVENT_CODE_EXTRACTOR_CODE, EventCodeExtractor, data, extractor);
        Err(ConfigSerializerError::UnknownExtractor)
    }

    fn read_filter_from_vec(data: &Vec<u8>, offset: &mut usize, filter_code: u16) -> Result<Box<dyn Filter>, ConfigSerializerError> {
        impl_item_read!(U8_INCREMENT_STATE_FILTER, U8IncrementStateFilter, data, offset, filter_code);
        impl_item_read!(U16_IS_EQUAL_FILTER_CODE, U16IsEqualFilter, data, offset, filter_code);
        impl_item_read!(U32_IS_EQUAL_STATE_FILTER_CODE, U32IsEqualStateFilter, data, offset, filter_code);
        impl_item_read!(U32_INCREMENT_STATE_FILTER_CODE, U32IncrementStateFilter, data, offset, filter_code);
        impl_item_read!(U32_SET_STATE_FILTER_CODE, U32SetStateFilter, data, offset, filter_code);
        impl_item_read!(FLIP_FLOP_FILTER_CODE, FlipFlopFilter, data, offset, filter_code);
        impl_item_read!(COUNT_FILTER_CODE, CountFilter, data, offset, filter_code);
        impl_item_read!(COUNT_STATE_FILTER_CODE, CountStateFilter, data, offset, filter_code);
        Err(ConfigSerializerError::UnknownFilter)
    }

    fn write_filter_to_vec(data: &mut Vec<u8>, filter: &Box<dyn Filter>) -> Result<(), ConfigSerializerError> {
        impl_item_write!(U8_INCREMENT_STATE_FILTER, U8IncrementStateFilter, data, filter);
        impl_item_write!(U16_IS_EQUAL_FILTER_CODE, U16IsEqualFilter, data, filter);
        impl_item_write!(U32_IS_EQUAL_STATE_FILTER_CODE, U32IsEqualStateFilter, data, filter);
        impl_item_write!(U32_INCREMENT_STATE_FILTER_CODE, U32IncrementStateFilter, data, filter);
        impl_item_write!(U32_SET_STATE_FILTER_CODE, U32SetStateFilter, data, filter);
        impl_item_write!(FLIP_FLOP_FILTER_CODE, FlipFlopFilter, data, filter);
        impl_item_write!(COUNT_FILTER_CODE, CountFilter, data, filter);
        impl_item_write!(COUNT_STATE_FILTER_CODE, CountStateFilter, data, filter);
        Err(ConfigSerializerError::UnknownFilter)
    }

    fn read_producer_from_vec(data: &Vec<u8>, offset: &mut usize, producer_code: u16) -> Result<Box<dyn Producer>, ConfigSerializerError> {
        impl_item_read!(NONE_PRODUCER_CODE, NoneProducer, data, offset, producer_code);
        impl_item_read!(BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE, BcmChangeBrightnessProducer, data, offset, producer_code);
        impl_item_read!(BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE, BcmChangeBrightnessStateProducer, data, offset, producer_code);
        Err(ConfigSerializerError::UnknownProducer)
    }

    fn write_producer_to_vec(data: &mut Vec<u8>, producer: &Box<dyn Producer>) -> Result<(), ConfigSerializerError> {
        impl_item_write!(NONE_PRODUCER_CODE, NoneProducer, data, producer);
        impl_item_write!(BCM_CHANGE_BRIGHTNESS_PRODUCER_CODE, BcmChangeBrightnessProducer, data, producer);
        impl_item_write!(BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE, BcmChangeBrightnessStateProducer, data, producer);
        Err(ConfigSerializerError::UnknownProducer)
    }
}