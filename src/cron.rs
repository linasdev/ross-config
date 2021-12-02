extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeSet;
use alloc::vec;
use alloc::vec::Vec;
use chrono::{DateTime, Datelike, Timelike, Utc};
use core::ops::AddAssign;

use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::{serialize_integer_to_vec, try_deserialize_integer_from_vec};

#[derive(Debug, PartialEq)]
pub enum CronField<T: Copy + Ord + AddAssign> {
    Including(BTreeSet<T>),
    Excluding(BTreeSet<T>),
    EveryFromTo(T, T, T),
    Any,
}

impl<T: Copy + Ord + AddAssign> CronField<T> {
    fn do_match(&self, value: T) -> bool {
        match self {
            CronField::Including(values) => values.contains(&value),
            CronField::Excluding(values) => values.contains(&value),
            CronField::EveryFromTo(every, from, to) => {
                let mut current_value = *from;

                loop {
                    if current_value == value {
                        break true;
                    }

                    current_value += *every;

                    if current_value > *to {
                        break false;
                    }
                }
            }
            CronField::Any => true,
        }
    }
}

impl Serialize for CronField<u8> {
    fn serialize(&self) -> Vec<u8> {
        match self {
            CronField::Including(values) => {
                let mut data = vec![0x00];

                serialize_integer_to_vec!(data, values.len(), u8);

                for value in values.iter() {
                    serialize_integer_to_vec!(data, *value, u8);
                }

                data
            }
            CronField::Excluding(values) => {
                let mut data = vec![0x01];

                serialize_integer_to_vec!(data, values.len(), u8);

                for value in values.iter() {
                    serialize_integer_to_vec!(data, *value, u8);
                }

                data
            }
            CronField::EveryFromTo(every, from, to) => vec![0x02, *every, *from, *to],
            CronField::Any => vec![0x03],
        }
    }
}

impl TryDeserialize for CronField<u8> {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 1 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => {
                if data.len() < 2 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut values = BTreeSet::new();
                let value_count = data[1];

                let mut offset = 2;

                for _ in 0..value_count {
                    values.insert(data[offset]);
                    offset += 1;
                }

                Ok(Box::new(CronField::Including(values)))
            }
            0x01 => {
                if data.len() < 2 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut values = BTreeSet::new();
                let value_count = data[1];

                let mut offset = 2;

                for _ in 0..value_count {
                    values.insert(data[offset]);
                    offset += 1;
                }

                Ok(Box::new(CronField::Excluding(values)))
            }
            0x02 => {
                if data.len() < 4 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                Ok(Box::new(CronField::EveryFromTo(data[1], data[2], data[3])))
            }
            0x03 => Ok(Box::new(CronField::Any)),
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
    }
}

impl Serialize for CronField<u16> {
    fn serialize(&self) -> Vec<u8> {
        match self {
            CronField::Including(values) => {
                let mut data = vec![0x00];

                serialize_integer_to_vec!(data, values.len(), u8);

                for value in values.iter() {
                    serialize_integer_to_vec!(data, *value, u16);
                }

                data
            }
            CronField::Excluding(values) => {
                let mut data = vec![0x01];

                serialize_integer_to_vec!(data, values.len(), u8);

                for value in values.iter() {
                    serialize_integer_to_vec!(data, *value, u16);
                }

                data
            }
            CronField::EveryFromTo(every, from, to) => {
                let mut data = vec![0x02];

                serialize_integer_to_vec!(data, *every, u16);
                serialize_integer_to_vec!(data, *from, u16);
                serialize_integer_to_vec!(data, *to, u16);

                data
            }
            CronField::Any => vec![0x03],
        }
    }
}

impl TryDeserialize for CronField<u16> {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 1 {
            return Err(ConfigSerializerError::WrongSize);
        }

        match data[0] {
            0x00 => {
                if data.len() < 2 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut values = BTreeSet::new();
                let value_count = data[1];

                let mut offset = 2;

                for _ in 0..value_count {
                    let value = try_deserialize_integer_from_vec!(data, offset, u16);
                    values.insert(value);
                }

                Ok(Box::new(CronField::Including(values)))
            }
            0x01 => {
                if data.len() < 2 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut values = BTreeSet::new();
                let value_count = data[1];

                let mut offset = 2;

                for _ in 0..value_count {
                    let value = try_deserialize_integer_from_vec!(data, offset, u16);
                    values.insert(value);
                }

                Ok(Box::new(CronField::Excluding(values)))
            }
            0x02 => {
                if data.len() < 7 {
                    return Err(ConfigSerializerError::WrongSize);
                }

                let mut offset = 1;

                let every = try_deserialize_integer_from_vec!(data, offset, u16);
                let from = try_deserialize_integer_from_vec!(data, offset, u16);
                #[allow(unused_assignments)]
                let to = try_deserialize_integer_from_vec!(data, offset, u16);

                Ok(Box::new(CronField::EveryFromTo(every, from, to)))
            }
            0x03 => Ok(Box::new(CronField::Any)),
            _ => Err(ConfigSerializerError::UnknownEnumVariant),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CronExpression {
    second: CronField<u8>,
    minute: CronField<u8>,
    hour: CronField<u8>,
    day_month: CronField<u8>,
    month: CronField<u8>,
    day_week: CronField<u8>,
    year: CronField<u16>,
}

impl CronExpression {
    fn do_match(&self, date_time: DateTime<Utc>) -> bool {
        if !self.second.do_match(date_time.second() as u8) {
            return false;
        }

        if !self.minute.do_match(date_time.minute() as u8) {
            return false;
        }

        if !self.hour.do_match(date_time.hour() as u8) {
            return false;
        }

        if !self.day_month.do_match(date_time.day0() as u8) {
            return false;
        }

        if !self.month.do_match(date_time.month0() as u8) {
            return false;
        }

        if !self
            .day_week
            .do_match(date_time.weekday().num_days_from_monday() as u8)
        {
            return false;
        }

        if !self.year.do_match(date_time.year() as u16) {
            return false;
        }

        return true;
    }
}

impl Serialize for CronExpression {
    fn serialize(&self) -> Vec<u8> {
        let mut data = vec![];

        let mut second = self.second.serialize();
        serialize_integer_to_vec!(data, second.len(), u8);
        data.append(&mut second);

        let mut minute = self.minute.serialize();
        serialize_integer_to_vec!(data, minute.len(), u8);
        data.append(&mut minute);

        let mut hour = self.hour.serialize();
        serialize_integer_to_vec!(data, hour.len(), u8);
        data.append(&mut hour);

        let mut day_month = self.day_month.serialize();
        serialize_integer_to_vec!(data, day_month.len(), u8);
        data.append(&mut day_month);

        let mut month = self.month.serialize();
        serialize_integer_to_vec!(data, month.len(), u8);
        data.append(&mut month);

        let mut day_week = self.day_week.serialize();
        serialize_integer_to_vec!(data, day_week.len(), u8);
        data.append(&mut day_week);

        let mut year = self.year.serialize();
        serialize_integer_to_vec!(data, year.len(), u8);
        data.append(&mut year);

        data
    }
}

impl TryDeserialize for CronExpression {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        if data.len() < 14 {
            return Err(ConfigSerializerError::WrongSize);
        }

        let mut offset = 0;

        let second_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
        let second = *CronField::<u8>::try_deserialize(&data[offset..offset + second_len])?;
        offset += second_len;

        let minute_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
        let minute = *CronField::<u8>::try_deserialize(&data[offset..offset + minute_len])?;
        offset += minute_len;

        let hour_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
        let hour = *CronField::<u8>::try_deserialize(&data[offset..offset + hour_len])?;
        offset += hour_len;

        let day_month_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
        let day_month = *CronField::<u8>::try_deserialize(&data[offset..offset + day_month_len])?;
        offset += day_month_len;

        let month_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
        let month = *CronField::<u8>::try_deserialize(&data[offset..offset + month_len])?;
        offset += month_len;

        let day_week_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
        let day_week = *CronField::<u8>::try_deserialize(&data[offset..offset + day_week_len])?;
        offset += day_week_len;

        let year_len = try_deserialize_integer_from_vec!(data, offset, u8) as usize;
        let year = *CronField::<u16>::try_deserialize(&data[offset..offset + year_len])?;

        Ok(Box::new(CronExpression {
            second,
            minute,
            hour,
            day_month,
            month,
            day_week,
            year,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_serialize_including_u8_test() {
        let mut values = BTreeSet::new();
        values.insert(0xab);
        values.insert(0x01);
        values.insert(0xff);

        let field = CronField::<u8>::Including(values);

        let expected_data = vec![0x00, 0x03, 0x01, 0xab, 0xff];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_including_u8_test() {
        let data = vec![0x00, 0x03, 0x01, 0xab, 0xff];

        let mut expected_values = BTreeSet::new();
        expected_values.insert(0xab);
        expected_values.insert(0x01);
        expected_values.insert(0xff);

        assert_eq!(
            CronField::<u8>::try_deserialize(&data),
            Ok(Box::new(CronField::<u8>::Including(expected_values))),
        )
    }

    #[test]
    fn field_serialize_excluding_u8_test() {
        let mut values = BTreeSet::new();
        values.insert(0xab);
        values.insert(0x01);
        values.insert(0xff);

        let field = CronField::<u8>::Excluding(values);

        let expected_data = vec![0x01, 0x03, 0x01, 0xab, 0xff];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_excluding_u8_test() {
        let data = vec![0x01, 0x03, 0x01, 0xab, 0xff];

        let mut expected_values = BTreeSet::new();
        expected_values.insert(0xab);
        expected_values.insert(0x01);
        expected_values.insert(0xff);

        assert_eq!(
            CronField::<u8>::try_deserialize(&data),
            Ok(Box::new(CronField::<u8>::Excluding(expected_values))),
        )
    }

    #[test]
    fn field_serialize_every_from_to_u8_test() {
        let field = CronField::<u8>::EveryFromTo(0xab, 0x01, 0xff);

        let expected_data = vec![0x02, 0xab, 0x01, 0xff];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_every_from_to_u8_test() {
        let data = vec![0x02, 0xab, 0x01, 0xff];

        assert_eq!(
            CronField::<u8>::try_deserialize(&data),
            Ok(Box::new(CronField::<u8>::EveryFromTo(0xab, 0x01, 0xff))),
        )
    }

    #[test]
    fn field_serialize_any_u8_test() {
        let field = CronField::<u8>::Any;

        let expected_data = vec![0x03];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_any_u8_test() {
        let data = vec![0x03];

        assert_eq!(
            CronField::<u8>::try_deserialize(&data),
            Ok(Box::new(CronField::<u8>::Any)),
        )
    }

    #[test]
    fn field_serialize_including_u16_test() {
        let mut values = BTreeSet::new();
        values.insert(0xabab);
        values.insert(0x0123);
        values.insert(0xffff);

        let field = CronField::<u16>::Including(values);

        let expected_data = vec![0x00, 0x03, 0x01, 0x23, 0xab, 0xab, 0xff, 0xff];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_including_u16_test() {
        let data = vec![0x00, 0x03, 0x01, 0x23, 0xab, 0xab, 0xff, 0xff];

        let mut expected_values = BTreeSet::new();
        expected_values.insert(0xabab);
        expected_values.insert(0x0123);
        expected_values.insert(0xffff);

        assert_eq!(
            CronField::<u16>::try_deserialize(&data),
            Ok(Box::new(CronField::<u16>::Including(expected_values))),
        )
    }

    #[test]
    fn field_serialize_excluding_u16_test() {
        let mut values = BTreeSet::new();
        values.insert(0xabab);
        values.insert(0x0123);
        values.insert(0xffff);

        let field = CronField::<u16>::Excluding(values);

        let expected_data = vec![0x01, 0x03, 0x01, 0x23, 0xab, 0xab, 0xff, 0xff];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_excluding_u16_test() {
        let data = vec![0x01, 0x03, 0x01, 0x23, 0xab, 0xab, 0xff, 0xff];

        let mut expected_values = BTreeSet::new();
        expected_values.insert(0xabab);
        expected_values.insert(0x0123);
        expected_values.insert(0xffff);

        assert_eq!(
            CronField::<u16>::try_deserialize(&data),
            Ok(Box::new(CronField::<u16>::Excluding(expected_values))),
        )
    }

    #[test]
    fn field_serialize_every_from_to_u16_test() {
        let field = CronField::<u16>::EveryFromTo(0xabab, 0x0123, 0xffff);

        let expected_data = vec![0x02, 0xab, 0xab, 0x01, 0x23, 0xff, 0xff];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_every_from_to_u16_test() {
        let data = vec![0x02, 0xab, 0xab, 0x01, 0x23, 0xff, 0xff];

        assert_eq!(
            CronField::<u16>::try_deserialize(&data),
            Ok(Box::new(CronField::<u16>::EveryFromTo(
                0xabab, 0x0123, 0xffff
            ))),
        )
    }

    #[test]
    fn field_serialize_any_u16_test() {
        let field = CronField::<u16>::Any;

        let expected_data = vec![0x03];

        assert_eq!(field.serialize(), expected_data)
    }

    #[test]
    fn field_try_deserialize_any_u16_test() {
        let data = vec![0x03];

        assert_eq!(
            CronField::<u16>::try_deserialize(&data),
            Ok(Box::new(CronField::<u16>::Any)),
        )
    }

    #[test]
    fn expression_serialize_test() {
        let mut included_values = BTreeSet::new();
        included_values.insert(0);
        included_values.insert(15);
        included_values.insert(59);

        let mut excluded_values = BTreeSet::new();
        excluded_values.insert(0);
        excluded_values.insert(15);
        excluded_values.insert(59);

        let expression = CronExpression {
            second: CronField::Including(included_values),
            minute: CronField::Excluding(excluded_values),
            hour: CronField::Any,
            day_month: CronField::Any,
            month: CronField::Any,
            day_week: CronField::Any,
            year: CronField::EveryFromTo(0x0123, 0xabab, 0xffff),
        };

        let expected_data = vec![
            5, 0, 3, 0, 15, 59, // second
            5, 1, 3, 0, 15, 59, // minute
            1, 3, // hour
            1, 3, // day (month)
            1, 3, // month
            1, 3, // day (week)
            7, 2, 0x01, 0x23, 0xab, 0xab, 0xff, 0xff, // year
        ];

        assert_eq!(expression.serialize(), expected_data);
    }

    #[test]
    fn expression_try_deserialize_test() {
        let data = vec![
            5, 0, 3, 0, 15, 59, // second
            5, 1, 3, 0, 15, 59, // minute
            1, 3, // hour
            1, 3, // day (month)
            1, 3, // month
            1, 3, // day (week)
            7, 2, 0x01, 0x23, 0xab, 0xab, 0xff, 0xff, // year
        ];

        let mut expected_included_values = BTreeSet::new();
        expected_included_values.insert(0);
        expected_included_values.insert(15);
        expected_included_values.insert(59);

        let mut expected_excluded_values = BTreeSet::new();
        expected_excluded_values.insert(0);
        expected_excluded_values.insert(15);
        expected_excluded_values.insert(59);

        assert_eq!(
            CronExpression::try_deserialize(&data),
            Ok(Box::new(CronExpression {
                second: CronField::Including(expected_included_values),
                minute: CronField::Excluding(expected_excluded_values),
                hour: CronField::EveryFromTo(1, 0, 59),
                day_month: CronField::EveryFromTo(1, 0, 30),
                month: CronField::EveryFromTo(1, 0, 11),
                day_week: CronField::EveryFromTo(1, 0, 6),
                year: CronField::EveryFromTo(0x0123, 0xabab, 0xffff),
            }))
        );
    }
}
