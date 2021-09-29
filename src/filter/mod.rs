use downcast_rs::{Downcast, impl_downcast};

use crate::state::StateManager;
use crate::Value;

pub mod state_filter;

mod u16_filter;
pub use u16_filter::*;

mod flip_flop_filter;
pub use flip_flop_filter::*;

mod count_filter;
pub use count_filter::*;

pub const U8_INCREMENT_STATE_FILTER: u16 = 0x0000;
pub const U16_IS_EQUAL_FILTER_CODE: u16 = 0x0001;
pub const U32_IS_EQUAL_STATE_FILTER_CODE: u16 = 0x0002;
pub const U32_INCREMENT_STATE_FILTER_CODE: u16 = 0x0003;
pub const U32_SET_STATE_FILTER_CODE: u16 = 0x0004;
pub const FLIP_FLOP_FILTER_CODE: u16 = 0x0005;
pub const COUNT_FILTER_CODE: u16 = 0x0006;
pub const COUNT_STATE_FILTER_CODE: u16 = 0x0007;

#[cfg_attr(feature = "std", typetag::serde(tag = "type"))]
pub trait Filter: Downcast {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool;
}

impl_downcast!(Filter);
