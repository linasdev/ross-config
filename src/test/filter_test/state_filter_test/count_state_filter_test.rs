use crate::Value;
use crate::filter::Filter;
use crate::filter::state_filter::*;
use crate::state::StateManager;

const VALUE_1: u32 = 0x0000_0000;
const VALUE_2: u8 = 0xff;

#[test]
fn count_state_filter_initial_zero_maximum_two_test() {
    let mut state_manager = StateManager::new();
    let state_index = state_manager.add_state(Value::U32(0));

    let mut filter = CountStateFilter::new(state_index, 2);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
}


#[test]
fn count_state_filter_initial_four_maximum_five_test() {
    let mut state_manager = StateManager::new();
    let state_index = state_manager.add_state(Value::U32(4));

    let mut filter = CountStateFilter::new(state_index, 5);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
}

#[test]
#[should_panic(expected = "Wrong value provided for count state filter.")]
fn count_state_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    let state_index = state_manager.add_state(Value::U32(VALUE_1));

    let mut filter = CountStateFilter::new(state_index, 5);

    filter.filter(&Value::U32(VALUE_1), &mut state_manager);
}


#[test]
#[should_panic(expected = "Wrong state value provided for count state filter.")]
fn count_state_filter_state_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    let state_index = state_manager.add_state(Value::U8(VALUE_2));

    let mut filter = CountStateFilter::new(state_index, 5);

    filter.filter(&Value::None, &mut state_manager);
}
