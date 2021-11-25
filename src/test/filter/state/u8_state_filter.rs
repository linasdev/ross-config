use crate::filter::state::*;
use crate::filter::Filter;
use crate::state::StateManager;
use crate::{ExtractorValue, StateValue};

const VALUE_1: u8 = 0x00;
const VALUE_2: u32 = 0xffff_ffff;

#[test]
fn u8_increment_state_filter_initial_zero_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U8(0));

    let mut filter = U8IncrementStateFilter::new(0);

    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(1));
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(2));
}

#[test]
fn u8_increment_state_filter_initial_seven_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U8(7));

    let mut filter = U8IncrementStateFilter::new(0);

    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(8));
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    assert_eq!(*state_manager.get_value(0).unwrap(), StateValue::U8(9));
}

#[test]
#[should_panic(expected = "Wrong value provided for u8 increment state filter.")]
fn u8_increment_state_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U8(VALUE_1));

    let mut filter = U8IncrementStateFilter::new(0);

    filter.filter(&ExtractorValue::U32(VALUE_2), &mut state_manager);
}

#[test]
#[should_panic(expected = "Wrong state value provided for u8 increment state filter.")]
fn u8_increment_state_filter_state_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U32(VALUE_2));

    let mut filter = U8IncrementStateFilter::new(0);

    filter.filter(&ExtractorValue::None, &mut state_manager);
}
