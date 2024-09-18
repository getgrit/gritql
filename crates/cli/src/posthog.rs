/// PostHog analytics client
///
/// This is largely copied from https://github.com/PostHog/posthog-rs/blob/main/src/lib.rs
///
/// (C) PostHog, Inc. 2024
use std::collections::HashMap;

use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use serde::Serialize;

lazy_static! {
    pub static ref POSTHOG_WRITE_KEY: String =
        String::from("phc_ksrztn1ogPbqUSUf1qRjhoC6GMzpmBm7iqSNhVzvor5");
    pub static ref POSTHOG_HOST: String = String::from("https://us.i.posthog.com/capture/");
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Event {
    event: String,
    properties: Properties,
    timestamp: Option<NaiveDateTime>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Properties {
    distinct_id: String,
    props: HashMap<String, serde_json::Value>,
}

impl Properties {
    fn new<S: Into<String>>(distinct_id: S) -> Self {
        Self {
            distinct_id: distinct_id.into(),
            props: Default::default(),
        }
    }
}

impl Event {
    pub fn new<S: Into<String>>(event: S, distinct_id: S) -> Self {
        Self {
            event: event.into(),
            properties: Properties::new(distinct_id),
            timestamp: None,
        }
    }

    /// Errors if `prop` fails to serialize
    pub fn insert_prop<K: Into<String>, P: Serialize>(
        &mut self,
        key: K,
        prop: P,
    ) -> Result<(), Error> {
        let as_json =
            serde_json::to_value(prop).map_err(|e| Error::Serialization(e.to_string()))?;
        let _ = self.properties.props.insert(key.into(), as_json);
        Ok(())
    }
}
