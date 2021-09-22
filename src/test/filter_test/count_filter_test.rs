use crate::Value;
use crate::filter::*;
use crate::state::StateManager;

const VALUE_1: u8 = 0xff;

#[test]
fn count_filter_initial_zero_maximum_two_test() {
    let mut state_manager = StateManager::new();
    let mut filter = CountFilter::new(0, 2);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
}


#[test]
fn count_filter_initial_four_maximum_five_test() {
    let mut state_manager = StateManager::new();
    let mut filter = CountFilter::new(4, 5);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
}

#[test]
#[should_panic(expected = "Wrong value provided for count filter.")]
fn count_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    let mut filter = CountFilter::new(0, 5);

    filter.filter(&Value::U8(VALUE_1), &mut state_manager);
}
