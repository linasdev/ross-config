use crate::filter::state::*;
use crate::filter::Filter;
use crate::state::StateManager;
use crate::Value;

const VALUE_1: u32 = 0x0000_0000;
const VALUE_2: u32 = 0xabab_abab;
const VALUE_3: u8 = 0xff;

#[test]
fn u32_is_equal_state_filter_values_equal_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(VALUE_1));

    let mut filter = U32IsEqualStateFilter::new(0, VALUE_1);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
}

#[test]
fn u32_is_equal_state_filter_values_not_equal_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(VALUE_1));

    let mut filter = U32IsEqualStateFilter::new(0, VALUE_2);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
}

#[test]
#[should_panic(expected = "Wrong value provided for u32 is equal state filter.")]
fn u32_is_equal_state_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(VALUE_1));

    let mut filter = U32IsEqualStateFilter::new(0, VALUE_2);

    filter.filter(&Value::U32(VALUE_1), &mut state_manager);
}

#[test]
#[should_panic(expected = "Wrong state value provided for u32 is equal state filter.")]
fn u32_is_equal_state_filter_state_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U8(VALUE_3));

    let mut filter = U32IsEqualStateFilter::new(0, VALUE_2);

    filter.filter(&Value::None, &mut state_manager);
}

#[test]
fn u32_increment_state_filter_initial_zero_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(0));

    let mut filter = U32IncrementStateFilter::new(0);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), Value::U32(1));
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), Value::U32(2));
}

#[test]
fn u32_increment_state_filter_initial_seven_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(7));

    let mut filter = U32IncrementStateFilter::new(0);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), Value::U32(8));
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), Value::U32(9));
}

#[test]
#[should_panic(expected = "Wrong value provided for u32 increment state filter.")]
fn u32_increment_state_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(VALUE_1));

    let mut filter = U32IncrementStateFilter::new(0);

    filter.filter(&Value::U8(VALUE_3), &mut state_manager);
}

#[test]
#[should_panic(expected = "Wrong state value provided for u32 increment state filter.")]
fn u32_increment_state_filter_state_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U8(VALUE_3));

    let mut filter = U32IncrementStateFilter::new(0);

    filter.filter(&Value::None, &mut state_manager);
}

#[test]
fn u32_set_state_filter_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(VALUE_1));

    let mut filter = U32SetStateFilter::new(0, VALUE_2);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), Value::U32(VALUE_2));
}

#[test]
#[should_panic(expected = "Wrong value provided for u32 set state filter.")]
fn u32_set_state_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, Value::U32(VALUE_1));

    let mut filter = U32SetStateFilter::new(0, VALUE_2);

    filter.filter(&Value::U32(VALUE_2), &mut state_manager);
}
