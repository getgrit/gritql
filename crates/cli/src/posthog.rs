/// PostHog analytics client
///
/// This is largely copied from https://github.com/PostHog/posthog-rs/blob/main/src/lib.rs
///
/// (C) PostHog, Inc. 2024
use std::{collections::HashMap, fmt::Formatter};

use anyhow::{bail, Result};
use chrono::NaiveDateTime;
use lazy_static::lazy_static;
use reqwest::{header::CONTENT_TYPE, Client as ReqwestClient};
use serde::Serialize;
use uuid::Uuid;

use crate::analytics::{AnalyticsEvent, AnalyticsPayload};

lazy_static! {
    pub static ref POSTHOG_WRITE_KEY: String =
        String::from("phc_ksrztn1ogPbqUSUf1qRjhoC6GMzpmBm7iqSNhVzvor5");
    pub static ref POSTHOG_HOST: String = String::from("https://us.i.posthog.com/capture/");
}

pub struct PostHogClient {
    client: ReqwestClient,
    api_endpoint: String,
}

impl PostHogClient {
    pub fn new(client: ReqwestClient) -> Self {
        Self {
            client,
            api_endpoint: POSTHOG_HOST.to_string(),
        }
    }

    async fn capture_internal(&self, event: PostHogEvent) -> Result<()> {
        let res = self
            .client
            .post(self.api_endpoint.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&event).expect("unwrap here is safe"))
            .send()
            .await?;
        if !res.status().is_success() {
            bail!("Failed to send event {}: {}", event.event, res.status());
        }
        Ok(())
    }

    pub async fn capture(&self, event: AnalyticsPayload<'_>) -> Result<()> {
        self.capture_internal(event.try_into()?).await
    }
}

// See https://posthog.com/docs/api/capture
#[derive(Serialize)]
struct PostHogEvent {
    api_key: String,
    event: String,
    distinct_id: String,
    properties: serde_json::Value,
    timestamp: Option<NaiveDateTime>,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
struct PostHogProperties {
    props: serde_json::Value,
}

impl TryFrom<AnalyticsPayload<'_>> for PostHogEvent {
    type Error = anyhow::Error;

    fn try_from(payload: AnalyticsPayload<'_>) -> Result<Self, Self::Error> {
        let distinct_id = payload.user_id.unwrap_or(Uuid::new_v4().to_string());

        let properties = serde_json::to_value(payload.properties)?;

        Ok(PostHogEvent {
            api_key: POSTHOG_WRITE_KEY.to_string(),
            event: payload.event.to_string(),
            distinct_id,
            properties,
            timestamp: None,
        })
    }
}
