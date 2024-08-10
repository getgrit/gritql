use std::env;

use crate::{info::AuthInfo, testing::get_testing_auth_info};

pub static ENV_VAR_GRIT_LOCAL_SERVER: &str = "GRIT_LOCAL_SERVER";
pub static ENV_VAR_GRIT_AUTH_TOKEN: &str = "GRIT_AUTH_TOKEN";
pub static ENV_VAR_GRIT_API_URL: &str = "GRIT_API_URL";
pub static DEFAULT_GRIT_API_URL: &str = "https://api2.grit.io";
pub static ENV_VAR_GRAPHQL_API_URL: &str = "GRAPHQL_API_URL";
pub static DEFAULT_GRAPHQL_API_URL: &str = "https://grit-prod-central.hasura.app/v1";
pub static ENV_VAR_GRIT_APP_URL: &str = "GRIT_APP_URL";
pub static DEFAULT_GRIT_APP_URL: &str = "https://app.grit.io";

pub fn get_grit_api_url() -> String {
    env::var(ENV_VAR_GRIT_API_URL).unwrap_or_else(|_| String::from(DEFAULT_GRIT_API_URL))
}

pub fn get_graphql_api_url() -> String {
    env::var(ENV_VAR_GRAPHQL_API_URL).unwrap_or_else(|_| String::from(DEFAULT_GRAPHQL_API_URL))
}

pub fn get_app_url() -> String {
    env::var(ENV_VAR_GRIT_APP_URL).unwrap_or_else(|_| String::from(DEFAULT_GRIT_APP_URL))
}

pub fn get_env_auth(allow_testing: bool) -> Option<AuthInfo> {
    let env_token = std::env::var(ENV_VAR_GRIT_AUTH_TOKEN).ok();
    if let Some(token) = env_token {
        return Some(AuthInfo::new(token.to_string()));
    }
    #[cfg(any(test, feature = "test-utils"))]
    if allow_testing {
        let testing = get_testing_auth_info();
        if let Ok(auth) = testing {
            return Some(auth);
        }
    }
    None
}

#[cfg(test)]
impl From<crate::info::AuthInfo> for marzano_util::runtime::LanguageModelAPI {
    fn from(auth_info: crate::info::AuthInfo) -> Self {
        let base_endpoint = env::var(ENV_VAR_GRIT_API_URL).unwrap();

        Self {
            base_endpoint,
            bearer_token: auth_info.access_token,
            can_cache: true,
        }
    }
}
