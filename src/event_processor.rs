extern crate alloc;

use alloc::vec::Vec;
use alloc::boxed::Box;

use crate::matcher::Matcher;
use crate::extractor::Extractor;
use crate::producer::Producer;

#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct EventProcessor {
    pub matchers: Vec<Matcher>,
    pub extractor: Box<dyn Extractor>,
    pub producer: Box<dyn Producer>,
}
