use anyhow::Result;
use std::env;

use lazy_static::lazy_static;
use serde_json::json;

use crate::{
    auth0::{AuthTokenResponseSuccess, AUTH0_API_AUDIENCE, AUTH0_TENANT_DOMAIN},
    info::AuthInfo,
};

/// Attempts to read a variable, first from Doppler, then from the environment.
///
/// If neither source has the variable, an error is returned.
fn get_config_var(var_name: &str) -> Result<String> {
    use std::process::Command;

    let output = Command::new("doppler")
        .args(["secrets", "get", var_name, "--plain"])
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let value = String::from_utf8_lossy(&output.stdout).trim().to_string();

            Ok(value)
        }
        _ => {
            let value = env::var(var_name)?;
            Ok(value)
        }
    }
}

fn get_existing_token() -> Result<AuthInfo> {
    let existing_token = get_config_var("API_TESTING_TOKEN")?;
    let info = AuthInfo {
        access_token: existing_token,
    };

    if info.is_expired()? {
        return Err(anyhow::anyhow!("existing token is expired"));
    }

    Ok(info)
}

fn get_new_tokens() -> Result<AuthInfo> {
    // Exchange client tokens for a test token
    let client_id = get_config_var("API_TESTING_CLIENT_ID")?;
    let client_secret = get_config_var("API_TESTING_CLIENT_SECRET")?;

    let client = reqwest::blocking::Client::new();

    let res = client
        .post(format!(
            "https://{}/oauth/oauth/token",
            AUTH0_TENANT_DOMAIN.as_str(),
        ))
        .header("Content-Type", "application/json")
        .json(&json!({
            "client_id": client_id,
            "client_secret": client_secret,
            "audience": AUTH0_API_AUDIENCE.as_str(),
            "grant_type": "client_credentials",
        }))
        .send()?;

    let body = res.json::<AuthTokenResponseSuccess>()?;

    Ok(AuthInfo {
        access_token: body.access_token,
    })
}

pub fn get_testing_auth_info() -> Result<AuthInfo> {
    let info = get_existing_token().or_else(|_| get_new_tokens())?;

    Ok(info)
}

lazy_static! {
    pub static ref TEST_AUTH_INFO: Result<AuthInfo> = get_testing_auth_info();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "eats up auth tokens, only enable when explicitly testing"]
    fn test_token_refresh() -> Result<()> {
        let auth_info = get_testing_auth_info().unwrap();
        assert!(!auth_info.is_expired()?);
        Ok(())
    }

    #[test]
    #[ignore = "tokens not used in open source tests"]
    fn test_uses_existing_token() -> Result<()> {
        let existing_info = get_existing_token().unwrap();
        let auth_info = TEST_AUTH_INFO.as_ref().unwrap();
        println!("auth_info: {}", auth_info);
        assert!(!auth_info.is_expired()?);
        assert_eq!(existing_info.access_token, auth_info.access_token);
        Ok(())
    }
}
