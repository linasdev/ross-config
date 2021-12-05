use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use crate::serializer::Serialize;
use crate::state_manager::StateManager;
use crate::ExtractorValue;

mod value_equal_to_const;
pub use value_equal_to_const::*;

mod state_equal_to_const;
pub use state_equal_to_const::*;

mod state_equal_to_value;
pub use state_equal_to_value::*;

mod increment_state_by_const;
pub use increment_state_by_const::*;

mod increment_state_by_value;
pub use increment_state_by_value::*;

mod decrement_state_by_const;
pub use decrement_state_by_const::*;

mod decrement_state_by_value;
pub use decrement_state_by_value::*;

mod set_state_to_const;
pub use set_state_to_const::*;

mod set_state_to_value;
pub use set_state_to_value::*;

mod flip_state;
pub use flip_state::*;

mod time_matches_cron_expression;
pub use time_matches_cron_expression::*;

mod state_more_than_const;
pub use state_more_than_const::*;

mod state_less_than_const;
pub use state_less_than_const::*;

mod set_state_to_state;
pub use set_state_to_state::*;

pub const VALUE_EQUAL_TO_CONST_FILTER_CODE: u16 = 0x0000;
pub const STATE_EQUAL_TO_CONST_FILTER_CODE: u16 = 0x0001;
pub const STATE_EQUAL_TO_VALUE_FILTER_CODE: u16 = 0x0002;
pub const INCREMENT_STATE_BY_CONST_FILTER_CODE: u16 = 0x0003;
pub const INCREMENT_STATE_BY_VALUE_FILTER_CODE: u16 = 0x0004;
pub const DECREMENT_STATE_BY_CONST_FILTER_CODE: u16 = 0x0005;
pub const DECREMENT_STATE_BY_VALUE_FILTER_CODE: u16 = 0x0006;
pub const SET_STATE_TO_CONST_FILTER_CODE: u16 = 0x0007;
pub const SET_STATE_TO_VALUE_FILTER_CODE: u16 = 0x0008;
pub const FLIP_STATE_FILTER_CODE: u16 = 0x0009;
pub const TIME_MATCHES_CRON_EXPRESSION_FILTER_CODE: u16 = 0x000a;
pub const STATE_MORE_THAN_CONST_FILTER_CODE: u16 = 0x000b;
pub const STATE_LESS_THAN_CONST_FILTER_CODE: u16 = 0x000c;
pub const SET_STATE_TO_STATE_FILTER_CODE: u16 = 0x000d;

#[derive(Debug, PartialEq)]
pub enum FilterError {
    WrongValueType,
    WrongStateType,
}

pub trait Filter: Downcast + Debug + Serialize {
    fn filter(
        &mut self,
        value: &ExtractorValue,
        state_manager: &mut StateManager,
    ) -> Result<bool, FilterError>;
    fn get_code(&self) -> u16;
}

impl_downcast!(Filter);
