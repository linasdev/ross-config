extern crate alloc;

use alloc::collections::BTreeMap;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use crate::Value;

pub struct StateManager {
    state: BTreeMap<u32, Value>,
    date_time: DateTime<Utc>,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            state: BTreeMap::new(),
            date_time: DateTime::from_utc(
                NaiveDateTime::new(
                    NaiveDate::from_ymd(1970, 1, 1),
                    NaiveTime::from_num_seconds_from_midnight(0, 0),
                ),
                Utc,
            ),
        }
    }

    pub fn get_value(&self, index: u32) -> Option<&Value> {
        self.state.get(&index)
    }

    pub fn set_value(&mut self, index: u32, value: Value) {
        self.state.insert(index, value);
    }

    pub fn get_date_time(&self) -> &DateTime<Utc> {
        &self.date_time
    }

    pub fn set_date_time(&mut self, date_time: DateTime<Utc>) {
        self.date_time = date_time;
    }
}
