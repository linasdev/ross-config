extern crate alloc;

use alloc::vec::Vec;

use crate::matcher::Matcher;
use crate::creator::Creator;

#[derive(Debug)]
pub struct EventProcessor {
    pub matchers: Vec<Matcher>,
    pub creators: Vec<Creator>,
}
