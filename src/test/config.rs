extern crate alloc;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;

use crate::config::{Config, ConfigSerializer, ConfigSerializerError};
use crate::event_processor::EventProcessor;
use crate::extractor::{EventCodeExtractor, NoneExtractor};
use crate::filter::U16IsEqualFilter;
use crate::matcher::Matcher;
use crate::producer::state::BcmChangeBrightnessStateProducer;
use crate::Value;

#[test]
fn serialize_config_serializer_test() {
    let mut initial_state = BTreeMap::new();
    initial_state.insert(0, Value::U8(0xff));

    let mut event_processors = vec![];
    event_processors.push(EventProcessor {
        matchers: vec![Matcher {
            extractor: Box::new(EventCodeExtractor::new()),
            filter: Box::new(U16IsEqualFilter::new(0xff)),
        }],
        extractor: Box::new(NoneExtractor::new()),
        producer: Box::new(BcmChangeBrightnessStateProducer::new(0xff, 0xff, 0)),
    });

    let config = Config {
        initial_state,
        event_processors,
    };

    let data = ConfigSerializer::serialize(&config).unwrap();

    let expected_data = vec![
        0x00, 0x00, 0x00, 0x01, // initial state count
        0x00, 0x00, 0x00, 0x00, // state_index
        0x01, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // state_value
        0x00, 0x00, 0x00, 0x01, // event processor count
        0x00, 0x00, 0x00, 0x01, // matcher count
        0x00, 0x01, // EVENT_CODE_EXTRACTOR_CODE
        0x00, 0x01, // U16_IS_EQUAL_FILTER_CODE
        0xff, 0x00, // value
        0x00, 0x00, // NONE_EXTRACTOR_CODE
        0x00, 0x02, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
        0xff, 0x00, // bcm_address
        0xff, 0x00, // channel
        0x00, 0x00, 0x00, 0x00, // state_index
    ];

    assert_eq!(data, expected_data);
}

#[test]
fn deserialize_config_serializer_wrong_size_test() {
    let data = vec![];

    let err = ConfigSerializer::deserialize(&data).unwrap_err();

    assert_eq!(ConfigSerializerError::WrongSize, err);
}

#[test]
fn deserialize_config_serializer_empty_test() {
    let data = vec![
        0x00, 0x00, 0x00, 0x00, // initial state count
        0x00, 0x00, 0x00, 0x00, // event processor count
    ];

    let config = ConfigSerializer::deserialize(&data).unwrap();

    assert_eq!(config.initial_state.len(), 0);
    assert_eq!(config.event_processors.len(), 0);
}

#[test]
fn deserialize_config_serializer_test() {
    let data = vec![
        0x00, 0x00, 0x00, 0x01, // initial state count
        0x00, 0x00, 0x00, 0x00, // state_index
        0x01, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // state_value
        0x00, 0x00, 0x00, 0x01, // event processor count
        0x00, 0x00, 0x00, 0x01, // matcher count
        0x00, 0x01, // EVENT_CODE_EXTRACTOR_CODE
        0x00, 0x01, // U16_IS_EQUAL_FILTER_CODE
        0xff, 0x00, // value
        0x00, 0x00, // NONE_EXTRACTOR_CODE
        0x00, 0x02, // BCM_CHANGE_BRIGHTNESS_STATE_PRODUCER_CODE
        0xff, 0x00, // bcm_address
        0xff, 0x00, // channel
        0x00, 0x00, 0x00, 0x00, // state_index
    ];

    let config = ConfigSerializer::deserialize(&data).unwrap();

    assert_eq!(config.initial_state.len(), 1);
    assert_eq!(*config.initial_state.get(&0).unwrap(), Value::U8(0xff));
    assert_eq!(config.event_processors.len(), 1);
}
