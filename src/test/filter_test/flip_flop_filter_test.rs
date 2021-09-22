use crate::filter::*;
use crate::state::StateManager;
use crate::Value;

const VALUE_1: u8 = 0xff;

#[test]
fn flip_flop_filter_initial_false_test() {
    let mut state_manager = StateManager::new();
    let mut filter = FlipFlopFilter::new(false);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
}

#[test]
fn flip_flop_filter_initial_true_test() {
    let mut state_manager = StateManager::new();
    let mut filter = FlipFlopFilter::new(true);

    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), false);
    assert_eq!(filter.filter(&Value::None, &mut state_manager), true);
}

#[test]
#[should_panic(expected = "Wrong value provided for flip flop filter.")]
fn flip_flop_filter_value_has_bad_type_test() {
    let mut state_manager = StateManager::new();
    let mut filter = FlipFlopFilter::new(true);

    filter.filter(&Value::U8(VALUE_1), &mut state_manager);
}
