/// PostHog analytics client
///
/// This is largely copied from https://github.com/PostHog/posthog-rs/blob/main/src/lib.rs
///
/// (C) PostHog, Inc. 2024
use std::{collections::HashMap, fmt::Formatter};

use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use reqwest::{header::CONTENT_TYPE, Client as ReqwestClient};
use serde::Serialize;

lazy_static! {
    pub static ref POSTHOG_WRITE_KEY: String =
        String::from("phc_ksrztn1ogPbqUSUf1qRjhoC6GMzpmBm7iqSNhVzvor5");
    pub static ref POSTHOG_HOST: String = String::from("https://us.i.posthog.com/capture/");
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Connection(msg) => write!(f, "Connection Error: {}", msg),
            Error::Serialization(msg) => write!(f, "Serialization Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug)]
pub enum Error {
    Connection(String),
    Serialization(String),
}

pub struct Client {
    client: ReqwestClient,
    api_key: String,
    api_endpoint: String,
}

impl Client {
    pub fn new(client: ReqwestClient) -> Self {
        Self {
            client,
            api_key: POSTHOG_WRITE_KEY.to_string(),
            api_endpoint: POSTHOG_HOST.to_string(),
        }
    }

    pub fn capture(&self, event: Event) -> Result<(), Error> {
        let inner_event = InnerEvent::new(event, self.api_key.clone());
        let _res = self
            .client
            .post(self.api_endpoint.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&inner_event).expect("unwrap here is safe"))
            .send()
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(())
    }

    pub fn capture_batch(&self, events: Vec<Event>) -> Result<(), Error> {
        for event in events {
            self.capture(event)?;
        }
        Ok(())
    }
}

// This exists so that the client doesn't have to specify the API key over and over
#[derive(Serialize)]
struct InnerEvent {
    api_key: String,
    event: String,
    properties: Properties,
    timestamp: Option<NaiveDateTime>,
}

impl InnerEvent {
    fn new(event: Event, api_key: String) -> Self {
        Self {
            api_key,
            event: event.event,
            properties: event.properties,
            timestamp: event.timestamp,
        }
    }
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
