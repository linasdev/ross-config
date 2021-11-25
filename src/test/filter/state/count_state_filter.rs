use crate::filter::state::*;
use crate::filter::Filter;
use crate::state::StateManager;
use crate::{ExtractorValue, StateValue};

const VALUE_1: u32 = 0x0000_0000;
const VALUE_2: u8 = 0xff;

#[test]
fn count_state_filter_initial_zero_maximum_two_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U32(0));

    let mut filter = CountStateFilter::new(0, 2);

    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
}

#[test]
fn count_state_filter_initial_four_maximum_five_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U32(4));

    let mut filter = CountStateFilter::new(0, 5);

    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), false);
    assert_eq!(filter.filter(&ExtractorValue::None, &mut state_manager), true);
}

#[test]
#[should_panic(expected = "Wrong value provided for count state filter.")]
fn count_state_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U32(VALUE_1));

    let mut filter = CountStateFilter::new(0, 5);

    filter.filter(&ExtractorValue::U32(VALUE_1), &mut state_manager);
}

#[test]
#[should_panic(expected = "Wrong state value provided for count state filter.")]
fn count_state_filter_state_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    state_manager.set_value(0, StateValue::U8(VALUE_2));

    let mut filter = CountStateFilter::new(0, 5);

    filter.filter(&ExtractorValue::None, &mut state_manager);
}
