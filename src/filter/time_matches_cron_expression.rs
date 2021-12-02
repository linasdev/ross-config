extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::cron::CronExpression;
use crate::filter::{Filter, FilterError, TIME_MATCHES_CRON_EXPRESSION_FILTER_CODE};
use crate::serializer::{ConfigSerializerError, Serialize, TryDeserialize};
use crate::state_manager::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct TimeMatchesCronExpressionFilter {
    expression: CronExpression,
}

impl TimeMatchesCronExpressionFilter {
    pub fn new(expression: CronExpression) -> Self {
        Self { expression }
    }
}

impl Filter for TimeMatchesCronExpressionFilter {
    fn filter(
        &mut self,
        _value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError> {
        Ok(self.expression.do_match(state_manager.get_date_time()))
    }

    fn get_code(&self) -> u16 {
        TIME_MATCHES_CRON_EXPRESSION_FILTER_CODE
    }
}

impl Serialize for TimeMatchesCronExpressionFilter {
    fn serialize(&self) -> Vec<u8> {
        self.expression.serialize()
    }
}

impl TryDeserialize for TimeMatchesCronExpressionFilter {
    fn try_deserialize(data: &[u8]) -> Result<Box<Self>, ConfigSerializerError> {
        let expression = *CronExpression::try_deserialize(data)?;

        Ok(Box::new(Self { expression }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use alloc::collections::BTreeSet;
    use alloc::vec;
    use chrono::DateTime;
    use core::str::FromStr;

    use crate::cron::CronField;

    #[test]
    fn time_matches_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_date_time(DateTime::from_str("2002-04-20T06:09:00+02:00").unwrap());

        let mut included_values = BTreeSet::new();
        included_values.insert(32);
        included_values.insert(9);
        included_values.insert(15);

        let mut excluded_values = BTreeSet::new();
        excluded_values.insert(32);
        excluded_values.insert(9);
        excluded_values.insert(15);

        let mut filter = TimeMatchesCronExpressionFilter::new(CronExpression {
            second: CronField::EveryFromTo(1, 0, 5),
            minute: CronField::Including(included_values),
            hour: CronField::Excluding(excluded_values),
            day_month: CronField::Any,
            month: CronField::Any,
            day_week: CronField::Any,
            year: CronField::EveryFromTo(15, 2002, 2002),
        });

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(true)
        );
    }

    #[test]
    fn time_does_not_match_test() {
        let mut state_manager = StateManager::new();
        state_manager.set_date_time(DateTime::from_str("2002-04-20T09:06:00+02:00").unwrap());

        let mut included_values = BTreeSet::new();
        included_values.insert(32);
        included_values.insert(9);
        included_values.insert(15);

        let mut excluded_values = BTreeSet::new();
        excluded_values.insert(32);
        excluded_values.insert(9);
        excluded_values.insert(15);

        let mut filter = TimeMatchesCronExpressionFilter::new(CronExpression {
            second: CronField::EveryFromTo(1, 0, 5),
            minute: CronField::Including(included_values),
            hour: CronField::Excluding(excluded_values),
            day_month: CronField::Any,
            month: CronField::Any,
            day_week: CronField::Any,
            year: CronField::EveryFromTo(15, 2002, 2002),
        });

        assert_eq!(
            filter.filter(&ExtractorValue::None, &mut state_manager),
            Ok(false)
        );
    }

    #[test]
    fn serialize_test() {
        let mut included_values = BTreeSet::new();
        included_values.insert(32);
        included_values.insert(9);
        included_values.insert(15);

        let mut excluded_values = BTreeSet::new();
        excluded_values.insert(32);
        excluded_values.insert(9);
        excluded_values.insert(15);

        let filter = TimeMatchesCronExpressionFilter::new(CronExpression {
            second: CronField::EveryFromTo(1, 0, 5),
            minute: CronField::Including(included_values),
            hour: CronField::Excluding(excluded_values),
            day_month: CronField::Any,
            month: CronField::Any,
            day_week: CronField::Any,
            year: CronField::EveryFromTo(15, 2002, 2002),
        });

        let expected_data = vec![
            4, 2, 1, 0, 5, // second
            5, 0, 3, 9, 15, 32, // minute
            5, 1, 3, 9, 15, 32, // hour
            1, 3, // day (month)
            1, 3, // month
            1, 3, // day (week)
            7, 2, 0, 15, 7, 210, 7, 210, // year
        ];

        assert_eq!(filter.serialize(), expected_data);
    }

    #[test]
    fn deserialize_test() {
        let data = vec![
            4, 2, 1, 0, 5, // second
            5, 0, 3, 9, 15, 32, // minute
            5, 1, 3, 9, 15, 32, // hour
            1, 3, // day (month)
            1, 3, // month
            1, 3, // day (week)
            7, 2, 0, 15, 7, 210, 7, 210, // year
        ];

        let mut expected_included_values = BTreeSet::new();
        expected_included_values.insert(32);
        expected_included_values.insert(9);
        expected_included_values.insert(15);

        let mut expected_excluded_values = BTreeSet::new();
        expected_excluded_values.insert(32);
        expected_excluded_values.insert(9);
        expected_excluded_values.insert(15);

        let filter = TimeMatchesCronExpressionFilter::new(CronExpression {
            second: CronField::EveryFromTo(1, 0, 5),
            minute: CronField::Including(expected_included_values),
            hour: CronField::Excluding(expected_excluded_values),
            day_month: CronField::Any,
            month: CronField::Any,
            day_week: CronField::Any,
            year: CronField::EveryFromTo(15, 2002, 2002),
        });

        assert_eq!(
            TimeMatchesCronExpressionFilter::try_deserialize(&data),
            Ok(Box::new(filter))
        );
    }

    #[test]
    fn deserialize_unknown_enum_variant_test() {
        let data = vec![
            0x0d, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab, 0xab,
        ];

        assert_eq!(
            TimeMatchesCronExpressionFilter::try_deserialize(&data),
            Err(ConfigSerializerError::UnknownEnumVariant)
        );
    }

    #[test]
    fn deserialize_wrong_size_test() {
        let data = vec![0x3, 0xab, 0xab, 0xab];

        assert_eq!(
            TimeMatchesCronExpressionFilter::try_deserialize(&data),
            Err(ConfigSerializerError::WrongSize),
        );
    }
}
