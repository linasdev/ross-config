extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::{serialize_integer_to_vec, try_deserialize_integer_from_vec};

#[derive(Debug, Clone, PartialEq)]
pub enum Peripheral {
    Bcm(BcmPeripheral, Vec<u16>),
    Relay(RelayPeripheral, Vec<u16>),
}

impl Serialize for Peripheral {
    fn serialize(&self) -> Vec<u8> {
        match self {
            Peripheral::Bcm(peripheral, gateway_addresses) => {
                let mut data = vec![0x00];

                let mut peripheral = peripheral.serialize();
                serialize_integer_to_vec!(data, peripheral.len(), u8);
                data.append(&mut peripheral);

                serialize_integer_to_vec!(data, gateway_addresses.len(), u8);

                for gateway_address in gateway_addresses {
                    serialize_integer_to_vec!(data, *gateway_address, u16);
                }

                data
            }
            Peripheral::Relay(peripheral, gateway_addresses) => {
                let mut data = vec![0x01];

                let mut peripheral = peripheral.serialize();
                serialize_integer_to_vec!(data, peripheral.len(), u8);
                data.append(&mut peripheral);

                serialize_integer_to_vec!(data, gateway_addresses.len(), u8);

                for gateway_address in gateway_addresses {
                    serialize_integer_to_vec!(data, *gateway_address, u16);
                }

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
            0x00 => {
                let mut offset = 1;

                let peripheral_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
                let peripheral =
                    *BcmPeripheral::try_deserialize(&data[offset..offset + peripheral_len])?;
                offset += peripheral_len;

                let mut gateway_addresses = vec![];
                let gateway_address_count =
                    try_deserialize_integer_from_vec!(data, offset, u8) as usize;

                gateway_addresses.reserve(gateway_address_count);

                for _ in 0..gateway_address_count {
                    let gateway_address = try_deserialize_integer_from_vec!(data, offset, u16);
                    gateway_addresses.push(gateway_address);
                }

                Ok(Box::new(Peripheral::Bcm(peripheral, gateway_addresses)))
            }
            0x01 => {
                let mut offset = 1;

                let peripheral_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
                let peripheral =
                    *RelayPeripheral::try_deserialize(&data[offset..offset + peripheral_len])?;
                offset += peripheral_len;

                let mut gateway_addresses = vec![];
                let gateway_address_count =
                    try_deserialize_integer_from_vec!(data, offset, u8) as usize;

                gateway_addresses.reserve(gateway_address_count);

                for _ in 0..gateway_address_count {
                    let gateway_address = try_deserialize_integer_from_vec!(data, offset, u16);
                    gateway_addresses.push(gateway_address);
                }

                Ok(Box::new(Peripheral::Relay(peripheral, gateway_addresses)))
            }
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
        let value = Peripheral::Bcm(BcmPeripheral::Rgb(0xab, 0x00, 0x01), vec![0x00, 0x01]);

        let expected_data = vec![
            0x00, 0x04, 0x01, 0xab, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x01,
        ];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn bcm_deserialize_test() {
        let data = vec![
            0x00, 0x04, 0x01, 0xab, 0x00, 0x01, 0x02, 0x00, 0x00, 0x00, 0x01,
        ];

        let expected_value = Box::new(Peripheral::Bcm(
            BcmPeripheral::Rgb(0xab, 0x00, 0x01),
            vec![0x00, 0x01],
        ));

        assert_eq!(Peripheral::try_deserialize(&data), Ok(expected_value));
    }

    #[test]
    fn relay_serialize_test() {
        let value = Peripheral::Relay(
            RelayPeripheral::DoubleExclusive(0xab, 0x00),
            vec![0x00, 0x01],
        );

        let expected_data = vec![0x01, 0x03, 0x01, 0xab, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01];

        assert_eq!(value.serialize(), expected_data);
    }

    #[test]
    fn relay_deserialize_test() {
        let data = vec![0x01, 0x03, 0x01, 0xab, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01];

        let expected_value = Box::new(Peripheral::Relay(
            RelayPeripheral::DoubleExclusive(0xab, 0x00),
            vec![0x00, 0x01],
        ));

        assert_eq!(Peripheral::try_deserialize(&data), Ok(expected_value));
    }
}
