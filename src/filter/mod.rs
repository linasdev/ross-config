use crate::Value;
use crate::state::StateManager;

pub mod state_filter;

mod u16_filter;
pub use u16_filter::*;

mod flip_flop_filter;
pub use flip_flop_filter::*;

mod count_filter;
pub use count_filter::*;

pub trait Filter {
    fn filter(&mut self, value: &Value, state_manager: &mut StateManager) -> bool;
}
