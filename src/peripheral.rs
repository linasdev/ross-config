extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Peripheral {
    Bcm(BcmPeripheral),
    Relay(RelayPeripheral),
}

impl Serialize for Peripheral {
    fn serialize(&self) -> Vec<u8> {
        match self {
            Peripheral::Bcm(peripheral) => {
                let mut data = vec![0x00];
                data.append(&mut peripheral.serialize());
                data
            }
            Peripheral::Relay(peripheral) => {
                let mut data = vec![0x01];
                data.append(&mut peripheral.serialize());
                data
            }
        }
    }
}

impl TryDeserialize for Peripheral {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => Ok(Box::new(Peripheral::Bcm(*BcmPeripheral::try_deserialize(
                &data[1..],
            )?))),
            0x01 => Ok(Box::new(Peripheral::Relay(
                *RelayPeripheral::try_deserialize(&data[1..])?,
            ))),
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BcmPeripheral {
    Single(u8),
    Rgb(u8, u8, u8),
    Rgbw(u8, u8, u8, u8),
}

impl Serialize for BcmPeripheral {
    fn serialize(&self) -> Vec<u8> {
        match *self {
            BcmPeripheral::Single(channel) => vec![0x00, channel],
            BcmPeripheral::Rgb(r, g, b) => vec![0x01, r, g, b],
            BcmPeripheral::Rgbw(r, g, b, w) => vec![0x02, r, g, b, w],
        }
    }
}

impl TryDeserialize for BcmPeripheral {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => Ok(Box::new(BcmPeripheral::Single(data[1]))),
            0x01 => {
                if data.len() < 4 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(BcmPeripheral::Rgb(data[1], data[2], data[3])))
            }
            0x02 => {
                if data.len() < 5 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(BcmPeripheral::Rgbw(
                    data[1], data[2], data[3], data[4],
                )))
            }
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RelayPeripheral {
    Single(u8),
    DoubleExclusive(u8, u8),
}

impl Serialize for RelayPeripheral {
    fn serialize(&self) -> Vec<u8> {
        match *self {
            RelayPeripheral::Single(channel) => vec![0x00, channel],
            RelayPeripheral::DoubleExclusive(channel1, channel2) => vec![0x01, channel1, channel2],
        }
    }
}

impl TryDeserialize for RelayPeripheral {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 2 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => Ok(Box::new(RelayPeripheral::Single(data[1]))),
            0x01 => {
                if data.len() < 3 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(RelayPeripheral::DoubleExclusive(data[1], data[2])))
            }
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcm_serialize_test() {
        let value = Peripheral::Bcm(BcmPeripheral::Rgb(0xab, 0x00, 0x01));

        let expected_data = vec![0x00, 0x01, 0xab, 0x00, 0x01];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn bcm_deserialize_test() {
        let data = vec![0x00, 0x01, 0xab, 0x00, 0x01];

        let expected_value = Box::new(Peripheral::Bcm(BcmPeripheral::Rgb(0xab, 0x00, 0x01)));

        assert_eq!(Peripheral::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn relay_serialize_test() {
        let value = Peripheral::Relay(RelayPeripheral::DoubleExclusive(0xab, 0x00));

        let expected_data = vec![0x01, 0x01, 0xab, 0x00];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn relay_deserialize_test() {
        let data = vec![0x01, 0x01, 0xab, 0x00];

        let expected_value = Box::new(Peripheral::Relay(RelayPeripheral::DoubleExclusive(
            0xab, 0x00,
        )));

        assert_eq!(Peripheral::try_deserialize(&data), Ok(expected_value));
    }
}
