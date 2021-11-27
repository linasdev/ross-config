use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use crate::state::StateManager;
use crate::ExtractorValue;

mod value_equal_to_const;
pub use value_equal_to_const::*;

mod state_equal_to_const;
pub use state_equal_to_const::*;

mod state_equal_to_value;
pub use state_equal_to_value::*;

mod state_increment_by_const;
pub use state_increment_by_const::*;

mod state_increment_by_value;
pub use state_increment_by_value::*;

mod set_state_to_const;
pub use set_state_to_const::*;

mod set_state_to_value;
pub use set_state_to_value::*;

mod flip_state;
pub use flip_state::*;

pub const VALUE_EQUAL_TO_CONST_FILTER_CODE: u16 = 0x0000;
pub const STATE_EQUAL_TO_CONST_FILTER_CODE: u16 = 0x0001;
pub const STATE_EQUAL_TO_VALUE_FILTER_CODE: u16 = 0x0002;
pub const STATE_INCREMENT_BY_CONST_FILTER_CODE: u16 = 0x0003;
pub const STATE_INCREMENT_BY_VALUE_FILTER_CODE: u16 = 0x0004;
pub const SET_STATE_TO_CONST_FILTER_CODE: u16 = 0x0005;
pub const SET_STATE_TO_VALUE_FILTER_CODE: u16 = 0x0006;
pub const FLIP_STATE_FILTER_CODE: u16 = 0x0007;

#[derive(Debug, PartialEq)]
pub enum FilterError {
    WrongValueType,
    WrongStateType,
}

pub trait Filter: Downcast + Debug {
    fn filter(
        &mut self,
        value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError>;
}

impl_downcast!(Filter);
