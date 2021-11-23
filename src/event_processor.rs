extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::extractor::Extractor;
use crate::matcher::Matcher;
use crate::producer::Producer;

#[derive(Debug)]
pub struct EventProcessor {
    pub matchers: Vec<Matcher>,
    pub extractor: Box<dyn Extractor>,
    pub producer: Box<dyn Producer>,
}
