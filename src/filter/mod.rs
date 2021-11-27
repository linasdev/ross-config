use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

use crate::state::StateManager;
use crate::ExtractorValue;

pub mod state;

mod u16_filter;
pub use u16_filter::*;

mod flip_flop_filter;
pub use flip_flop_filter::*;

mod count_filter;
pub use count_filter::*;

mod u8_filter;
pub use u8_filter::*;

pub const U8_INCREMENT_STATE_FILTER: u16 = 0x0000;
pub const U16_IS_EQUAL_FILTER_CODE: u16 = 0x0001;
pub const U32_IS_EQUAL_STATE_FILTER_CODE: u16 = 0x0002;
pub const U32_INCREMENT_STATE_FILTER_CODE: u16 = 0x0003;
pub const U32_SET_STATE_FILTER_CODE: u16 = 0x0004;
pub const FLIP_FLOP_FILTER_CODE: u16 = 0x0005;
pub const COUNT_FILTER_CODE: u16 = 0x0006;
pub const COUNT_STATE_FILTER_CODE: u16 = 0x0007;
pub const BOOL_IS_EQUAL_STATE_FILTER_CODE: u16 = 0x0008;
pub const BOOL_SET_STATE_FILTER_CODE: u16 = 0x0009;
pub const U8_IS_EQUAL_FILTER_CODE: u16 = 0x000a;
pub const U8_SET_STATE_FILTER_CODE: u16 = 0x000b;
pub const U8_SET_STATE_FROM_VALUE_FILTER_CODE: u16 = 0x000c;
pub const BOOL_FLIP_STATE_FILTER_CODE: u16 = 0x000d;

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
