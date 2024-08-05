use anyhow::Result;
use lazy_static::lazy_static;
use log::debug;
use serde::Deserialize;
use serde::Serialize;
use std::time::Instant;
use tokio::time::{interval, Duration};

use crate::info::AuthInfo;

lazy_static! {
    pub static ref AUTH0_API_AUDIENCE: String = String::from("https://api2.grit.io");

    // prod values:
    pub static ref AUTH0_TENANT_DOMAIN: String = String::from("auth0.grit.io");
    // Note: this value is not secret and is fine to share publicly
    static ref AUTH0_CLI_CLIENT_ID: String = String::from("OAIbQpTHo0IxmMHwsi3p1hp6StA0n4Fo");
}

#[derive(Serialize)]
struct AuthCodeRequest {
    client_id: String,
    scope: String,
    audience: String,
}

#[derive(Debug, Deserialize)]
struct AuthCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri_complete: String,
    /// indicates the lifetime (in seconds) of the device_code and user_code
    expires_in: u64,
    /// indicates the interval (in seconds) at which the app should poll the token URL to request a token.
    interval: u64,
}

#[derive(Serialize)]
struct AuthTokenParams {
    grant_type: String,
    device_code: String,
    client_id: String,
}

#[derive(Debug, Deserialize)]
struct AuthTokenResponsePending {
    error: String,
    error_description: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[allow(dead_code)]
pub struct AuthTokenResponseSuccess {
    pub(crate) access_token: String,
    /// The refresh token, which can be used to obtain new access tokens using the same authorization grant
    pub(crate) refresh_token: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum AuthTokenResponse {
    Pending(AuthTokenResponsePending),
    Success(AuthTokenResponseSuccess),
}

pub struct AuthSession {
    init_request: AuthCodeResponse,
    token_response: Option<AuthTokenResponseSuccess>,
    started_at: Instant,
}

impl AuthSession {
    fn new(init_request: AuthCodeResponse) -> Self {
        Self {
            init_request,
            started_at: Instant::now(),
            token_response: None,
        }
    }

    pub fn user_code(&self) -> &str {
        &self.init_request.user_code
    }

    pub fn verify_url(&self) -> &str {
        &self.init_request.verification_uri_complete
    }

    pub fn token(self) -> Result<AuthInfo> {
        match self.token_response {
            Some(token) => Ok(AuthInfo::from(token)),
            None => Err(anyhow::anyhow!(
                "No Grit token available, please run grit auth login"
            )),
        }
    }

    /// Polls the Auth0 API to check if the user has completed the authentication process.
    pub async fn poll(&mut self) -> Result<()> {
        let client = reqwest::Client::new();

        let mut interval = interval(Duration::from_secs(self.init_request.interval));

        loop {
            interval.tick().await;

            let new_now = Instant::now();
            let elapsed = new_now.duration_since(self.started_at);
            // If we have exceeded the interval, then we should fail
            if (elapsed.as_secs()) > self.init_request.expires_in {
                return Err(anyhow::anyhow!("Authentication timed out"));
            }

            debug!("Polling Auth0 API... {}", self.init_request.expires_in);

            let params = AuthTokenParams {
                grant_type: String::from("urn:ietf:params:oauth:grant-type:device_code"),
                device_code: self.init_request.device_code.clone(),
                client_id: AUTH0_CLI_CLIENT_ID.as_str().to_string(),
            };

            let res = client
                .post(format!(
                    "https://{}/oauth/oauth/token",
                    AUTH0_TENANT_DOMAIN.as_str(),
                ))
                .header("Content-Type", "application/x-www-form-urlencoded")
                .form(&params)
                .send()
                .await?;

            let body = res.json::<AuthTokenResponse>().await?;
            match body {
                AuthTokenResponse::Pending(pending) => {
                    if pending.error == "authorization_pending" {
                        continue;
                    }
                    return Err(anyhow::anyhow!(
                        "Authentication failed: {}",
                        pending.error_description
                    ));
                }
                AuthTokenResponse::Success(success) => {
                    println!("Auth token response: {:?}", success);

                    self.token_response = Some(success);

                    break;
                }
            }
        }

        Ok(())
    }
}

/// Starts the authentication process with Auth0.
pub async fn start_auth() -> Result<AuthSession> {
    let client = reqwest::Client::new();

    let params = AuthCodeRequest {
        client_id: AUTH0_CLI_CLIENT_ID.as_str().to_string(),
        scope: "offline_access".to_string(),
        audience: AUTH0_API_AUDIENCE.as_str().to_string(),
    };

    let res = client
        .post(format!(
            "https://{}/oauth/device/code",
            AUTH0_TENANT_DOMAIN.as_str(),
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    let body = res.json::<AuthCodeResponse>().await?;

    Ok(AuthSession::new(body))
}

/// Refreshes an AuthInfo using the refresh token.
pub async fn refresh_token(auth_info: &AuthInfo) -> Result<AuthInfo> {
    if auth_info.refresh_token.is_none() {
        return Err(anyhow::anyhow!("No refresh token available"));
    }

    let client = reqwest::Client::new();

    let params = [
        ("grant_type", "refresh_token"),
        ("client_id", AUTH0_CLI_CLIENT_ID.as_str()),
        ("refresh_token", auth_info.refresh_token.as_ref().unwrap()),
    ];

    let res = client
        .post(format!(
            "https://{}/oauth/token",
            AUTH0_TENANT_DOMAIN.as_str(),
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&params)
        .send()
        .await?;

    let body = res.json::<AuthTokenResponseSuccess>().await?;

    let mut info = AuthInfo::from(body);

    // If the new response doesn't include a refresh token, use the old one
    if info.refresh_token.is_none() {
        info.refresh_token = auth_info.refresh_token.clone();
    }

    Ok(info)
}
