use crate::filter::Filter;
use crate::state::StateManager;
use crate::ExtractorValue;

#[repr(C)]
#[derive(Debug)]
pub struct U16IsEqualFilter {
    value: u16,
}

impl U16IsEqualFilter {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

impl Filter for U16IsEqualFilter {
    fn filter(&mut self, value: &ExtractorValue, _state_manager: &mut StateManager) -> bool {
        let value = match value {
            ExtractorValue::U16(value) => value,
            _ => {
                panic!("Wrong value provided for u16 is equal filter.");
            }
        };

        return *value == self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    const VALUE_1: u16 = 0x0000;
    const VALUE_2: u16 = 0xabab;
    const VALUE_3: u8 = 0xff;
    
    #[test]
    fn u16_is_equal_filter_values_equal_test() {
        let mut state_manager = StateManager::new();
        let mut filter = U16IsEqualFilter::new(VALUE_1);
    
        assert_eq!(
            filter.filter(&ExtractorValue::U16(VALUE_1), &mut state_manager),
            true
        );
    }
    
    #[test]
    fn u16_is_equal_filter_values_not_equal_test() {
        let mut state_manager = StateManager::new();
        let mut filter = U16IsEqualFilter::new(VALUE_1);
    
        assert_eq!(
            filter.filter(&ExtractorValue::U16(VALUE_2), &mut state_manager),
            false
        );
    }
    
    #[test]
    #[should_panic(expected = "Wrong value provided for u16 is equal filter.")]
    fn u16_is_equal_filter_value_has_bad_type_test() {
        let mut state_manager = StateManager::new();
        let mut filter = U16IsEqualFilter::new(VALUE_1);
    
        filter.filter(&ExtractorValue::U8(VALUE_3), &mut state_manager);
    }    
}
