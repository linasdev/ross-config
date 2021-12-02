extern crate alloc;

use alloc::vec::Vec;

use crate::creator::Creator;
use crate::matcher::Matcher;

#[derive(Debug)]
pub struct EventProcessor {
    pub matcher: Matcher,
    pub creators: Vec<Creator>,
}
