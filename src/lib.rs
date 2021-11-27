#![no_std]
extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryInto;

use ross_protocol::packet::Packet;

use crate::serializer::{TryDeserialize, Serialize, ConfigSerializerError};

pub mod config;
pub mod creator;
pub mod event_processor;
pub mod extractor;
pub mod filter;
pub mod matcher;
pub mod producer;
pub mod serializer;
pub mod state_manager;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    U8(u8),
    U16(u16),
    U32(u32),
    Bool(bool),
}

impl Serialize for Value {
    fn serialize(&self) -> Vec<u8> {
        match *self {
            Value::U8(value) => {
                vec![
                    0x00,
                    value,
                ]
            },
            Value::U16(value) => {
                let bytes = value.to_be_bytes();

                vec![
                    0x01,
                    bytes[0],
                    bytes[1],
                ]
            },
            Value::U32(value) => {
                let bytes = value.to_be_bytes();
                
                vec![
                    0x02,
                    bytes[0],
                    bytes[1],
                    bytes[2],
                    bytes[3],
                ]
            },
            Value::Bool(value) => {
                vec![
                    0x03,
                    if value {
                        0x01
                    } else {
                        0x00
                    },
                ]
            }
        }
    }
}

impl TryDeserialize for Value {
    fn try_deserialize(data: &[u8]) -> Result<Self, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => {
                Ok(Value::U8(data[1]))
            },
            0x01 => {
                if data.len() < 3 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let value = u16::from_be_bytes(data[1..=2].try_into().unwrap());

                Ok(Value::U16(value))
            },
            0x02 => {
                if data.len() < 5 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let value = u32::from_be_bytes(data[1..=4].try_into().unwrap());

                Ok(Value::U32(value))
            },
            0x03 => {
                Ok(Value::Bool(data[1] != 0x00))
            },
            _ => {
                Err(ConfigSerializerError::UnknownEnumVariant)
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExtractorValue<'a> {
    None,
    U8(u8),
    U16(u16),
    U32(u32),
    Bool(bool),
    Packet(&'a Packet),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_serialize_test() {
        let value = Value::U8(0xab);

        let expected_data = vec![
            0x00,
            0xab,
        ];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn u8_deserialize_test() {
        let data = vec![
            0x00,
            0xab,
        ];

        let expected_value = Value::U8(0xab);

        assert_eq!(Value::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn u16_serialize_test() {
        let value = Value::U16(0xabab);

        let expected_data = vec![
            0x01,
            0xab,
            0xab,
        ];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn u16_deserialize_test() {
        let data = vec![
            0x01,
            0xab,
            0xab,
        ];

        let expected_value = Value::U16(0xabab);

        assert_eq!(Value::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn u32_serialize_test() {
        let value = Value::U32(0xabab_abab);

        let expected_data = vec![
            0x02,
            0xab,
            0xab,
            0xab,
            0xab,
        ];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn u32_deserialize_test() {
        let data = vec![
            0x02,
            0xab,
            0xab,
            0xab,
            0xab,
        ];

        let expected_value = Value::U32(0xabab_abab);

        assert_eq!(Value::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn bool_serialize_test() {
        let value = Value::Bool(true);

        let expected_data = vec![
            0x03,
            0x01,
        ];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn bool_deserialize_test() {
        let data = vec![
            0x03,
            0x01,
        ];

        let expected_value = Value::Bool(true);

        assert_eq!(Value::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn wrong_size_test() {
        let data = vec![
            0x02,
            0xab,
            0xab,
            0xab,
        ];

        assert_eq!(Value::try_deserialize(&data), Err(ConfigSerializerError::WrongSize));
    }

    #[test]
    fn unknown_enum_variant_test() {
        let data = vec![
            0x04,
            0x01,
        ];

        assert_eq!(Value::try_deserialize(&data), Err(ConfigSerializerError::UnknownEnumVariant));
    }
}
