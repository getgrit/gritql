use std::env;

pub static ENV_VAR_GRIT_AUTH_TOKEN: &str = "GRIT_AUTH_TOKEN";
pub static ENV_VAR_GRIT_API_URL: &str = "GRIT_API_URL";
pub static DEFAULT_GRIT_API_URL: &str = "https://api-gateway-prod-6et7uue.uc.gateway.dev";

pub fn get_grit_api_url() -> String {
    env::var(ENV_VAR_GRIT_API_URL).unwrap_or_else(|_| String::from(DEFAULT_GRIT_API_URL))
}

#[cfg(test)]
impl From<crate::info::AuthInfo> for marzano_util::runtime::LanguageModelAPI {
    fn from(auth_info: crate::info::AuthInfo) -> Self {
        let base_endpoint = env::var("GRIT_API_URL").unwrap();

        Self {
            base_endpoint,
            bearer_token: auth_info.access_token,
            can_cache: true,
        }
    }
}
