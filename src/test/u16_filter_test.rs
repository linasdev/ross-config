use crate::Value;
use crate::filter::*;
use crate::state::StateManager;

const VALUE_1: u16 = 0x0000;
const VALUE_2: u16 = 0xabab;
const VALUE_3: u8 = 0xff;

#[test]
fn u16_is_equal_filter_values_equal_test() {
    let mut state_manager = StateManager::new();
    let mut filter = U16IsEqualFilter::new(VALUE_1);

    assert_eq!(filter.filter(&Value::U16(VALUE_1), &mut state_manager), true);
}

#[test]
fn u16_is_equal_filter_values_not_equal_test() {
    let mut state_manager = StateManager::new();
    let mut filter = U16IsEqualFilter::new(VALUE_1);

    assert_eq!(filter.filter(&Value::U16(VALUE_2), &mut state_manager), false);
}

#[test]
#[should_panic(expected = "Wrong value provided for u16 is equal filter.")]
fn u16_is_equal_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    let mut filter = U16IsEqualFilter::new(VALUE_1);

    filter.filter(&Value::U8(VALUE_3), &mut state_manager);
}
