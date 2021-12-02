extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::vec;

use ross_protocol::packet::Packet;

use crate::{serialize_integer_to_vec, try_deserialize_integer_from_vec};
use crate::extractor::{Extractor, ExtractorError};
use crate::filter::{Filter, FilterError};
use crate::state_manager::StateManager;
use crate::serializer::{Serialize, TryDeserialize, ConfigSerializerError, ConfigSerializer};

#[derive(Debug)]
pub enum MatcherError {
    ExtractorError(ExtractorError),
    FilterError(FilterError),
}

#[derive(Debug)]
pub enum Matcher {
    Single {
        extractor: Box<dyn Extractor>,
        filter: Box<dyn Filter>,
    },
    Not(Box<Matcher>),
    Or(Box<Matcher>, Box<Matcher>),
    And(Box<Matcher>, Box<Matcher>),
}

impl Matcher {
    pub fn do_match(
        &mut self,
        packet: &Packet,
        state_manager: &mut StateManager,
    ) -> Result<bool, MatcherError> {
        match self {
            Matcher::Single { extractor, filter } => {
                let value = extractor
                    .extract(packet)
                    .map_err(|err| MatcherError::ExtractorError(err))?;
                let result = filter
                    .filter(&value, state_manager)
                    .map_err(|err| MatcherError::FilterError(err))?;

                Ok(result)
            }
            Matcher::Not(matcher) => Ok(!matcher.do_match(packet, state_manager)?),
            Matcher::Or(matcher1, matcher2) => Ok(matcher1.do_match(packet, state_manager)?
                || matcher2.do_match(packet, state_manager)?),
            Matcher::And(matcher1, matcher2) => Ok(matcher1.do_match(packet, state_manager)?
                && matcher2.do_match(packet, state_manager)?),
        }
    }
}

impl Serialize for Matcher {
    fn serialize(&self) -> Vec<u8> {
        match self {
            Matcher::Single{extractor, filter} => {
                let mut data = vec![0x00];

                serialize_integer_to_vec!(data, extractor.get_code(), u16);
                let mut extractor = extractor.serialize();
                serialize_integer_to_vec!(data, extractor.len() as u8, u8);
                data.append(&mut extractor);

                serialize_integer_to_vec!(data, filter.get_code(), u16);
                let mut filter = filter.serialize();
                serialize_integer_to_vec!(data, filter.len() as u8, u8);
                data.append(&mut filter);

                data
            },
            Matcher::Not(matcher) => {
                let mut data = vec![0x01];

                let mut matcher = matcher.serialize();
                serialize_integer_to_vec!(data, matcher.len() as u32, u32);
                data.append(&mut matcher);

                data
            },
            Matcher::Or(matcher1, matcher2) => {
                let mut data = vec![0x02];

                let mut matcher1 = matcher1.serialize();
                serialize_integer_to_vec!(data, matcher1.len() as u32, u32);
                data.append(&mut matcher1);

                let mut matcher2 = matcher2.serialize();
                serialize_integer_to_vec!(data, matcher2.len() as u32, u32);
                data.append(&mut matcher2);

                data
            },
            Matcher::And(matcher1, matcher2) => {
                let mut data = vec![0x03];

                let mut matcher1 = matcher1.serialize();
                serialize_integer_to_vec!(data, matcher1.len() as u32, u32);
                data.append(&mut matcher1);

                let mut matcher2 = matcher2.serialize();
                serialize_integer_to_vec!(data, matcher2.len() as u32, u32);
                data.append(&mut matcher2);

                data
            }
        }
    }
}

impl TryDeserialize for Matcher {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => {
                if data.len() < 7 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut offset = 1;

                let extractor_code = try_deserialize_integer_from_vec!(data, offset, u16);
                let extractor_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
                let extractor = ConfigSerializer::try_deserialize_extractor_from_vec(
                    &data[offset..offset + extractor_len],
                    extractor_code,
                )?;
                offset += extractor_len;

                let filter_code = try_deserialize_integer_from_vec!(data, offset, u16);
                let filter_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
                let filter =
                ConfigSerializer::try_deserialize_filter_from_vec(&data[offset..offset + filter_len], filter_code)?;

                Ok(Box::new(Matcher::Single {
                    extractor,
                    filter,
                }))
            },
            0x01 => {
                if data.len() < 5 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut offset = 1;

                let matcher_len = try_deserialize_integer_from_vec!(data, offset, u32) as usize;
                let matcher = Matcher::try_deserialize(&data[offset..offset + matcher_len])?;

                Ok(Box::new(Matcher::Not(matcher)))
            }
            0x02 => {
                if data.len() < 9 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut offset = 1;

                let matcher1_len = try_deserialize_integer_from_vec!(data, offset, u32) as usize;
                let matcher1 = Matcher::try_deserialize(&data[offset..offset + matcher1_len])?;
                offset += matcher1_len;

                let matcher2_len = try_deserialize_integer_from_vec!(data, offset, u32) as usize;
                let matcher2 = Matcher::try_deserialize(&data[offset..offset + matcher2_len])?;

                Ok(Box::new(Matcher::Or(matcher1, matcher2)))
            }
            0x03 => {
                if data.len() < 9 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut offset = 1;

                let matcher1_len = try_deserialize_integer_from_vec!(data, offset, u32) as usize;
                let matcher1 = Matcher::try_deserialize(&data[offset..offset + matcher1_len])?;
                offset += matcher1_len;

                let matcher2_len = try_deserialize_integer_from_vec!(data, offset, u32) as usize;
                let matcher2 = Matcher::try_deserialize(&data[offset..offset + matcher2_len])?;

                Ok(Box::new(Matcher::And(matcher1, matcher2)))
            }
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
    }
}
