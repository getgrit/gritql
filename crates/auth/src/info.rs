use chrono::DateTime;
use chrono::Utc;
use grit_util::error::GritPatternError;
use grit_util::error::GritResult;
use marzano_util::base64::decode_to_string;

#[derive(Clone, Debug)]
pub struct AuthInfo {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

impl From<crate::auth0::AuthTokenResponseSuccess> for AuthInfo {
    fn from(auth: crate::auth0::AuthTokenResponseSuccess) -> Self {
        Self {
            access_token: auth.access_token,
            refresh_token: auth.refresh_token,
        }
    }
}

#[derive(serde::Deserialize, Debug)]
struct HasuraClaims {
    #[serde(rename = "x-hasura-raw-nickname")]
    nickname: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
struct AuthInfoPayload {
    exp: u64,
    sub: String,
    #[serde(rename = "https://hasura.io/jwt/claims")]
    hasura_claims: Option<HasuraClaims>,
}

impl AuthInfo {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            refresh_token: None,
        }
    }

    fn get_payload(&self) -> GritResult<AuthInfoPayload> {
        let basics = self
            .access_token
            .split('.')
            .nth(1)
            .ok_or(GritPatternError::new(
                "Invalid token format, expected 3 parts separated by '.'",
            ))?;
        let raw = decode_to_string(basics.as_bytes())?;

        let payload: AuthInfoPayload =
            serde_json::from_str(&raw).map_err(|e| GritPatternError::new(e.to_string()))?;

        Ok(payload)
    }

    pub fn get_expiry(&self) -> GritResult<DateTime<Utc>> {
        let payload = self.get_payload()?;
        let expiry = DateTime::<Utc>::from_timestamp(payload.exp as i64, 0)
            .ok_or(GritPatternError::new("Invalid timestamp"))?;
        Ok(expiry)
    }

    pub fn get_user_name(&self) -> GritResult<Option<String>> {
        let payload = self.get_payload()?;
        Ok(payload.hasura_claims.and_then(|c| c.nickname))
    }

    pub fn get_user_id(&self) -> GritResult<String> {
        let payload = self.get_payload()?;
        Ok(payload.sub)
    }

    pub fn is_expired(&self) -> GritResult<bool> {
        let expiry = self.get_expiry()?;
        Ok(expiry < Utc::now())
    }
}

impl std::fmt::Display for AuthInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let payload = self.get_payload().unwrap();
        write!(
            f,
            "AuthInfo:\n  Token: {},\nExpires: {:?}\n,  Payload: {:?}",
            self.access_token,
            self.get_expiry(),
            payload
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_user_name() {
        // This token is safe, it isn't signed by a real authority - only use for testing
        let jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJodHRwczovL2hhc3VyYS5pby9qd3QvY2xhaW1zIjp7IngtaGFzdXJhLWRlZmF1bHQtcm9sZSI6InVzZXIiLCJ4LWhhc3VyYS1hbGxvd2VkLXJvbGVzIjpbInVzZXIiXSwieC1oYXN1cmEtdXNlci1pZCI6ImdpdGh1YnwxNjI3ODAxIiwieC1oYXN1cmEtcmF3LW5pY2tuYW1lIjoibW9yZ2FudGUiLCJ4LWhhc3VyYS11c2VyLXRlbmFudCI6ImdpdGh1YiIsIngtaGFzdXJhLWF1dGgtcHJvdmlkZXIiOiJnaXRodWIiLCJ4LWhhc3VyYS11c2VyLW5pY2tuYW1lIjoiZ2l0aHVifG1vcmdhbnRlIn0sImlzcyI6Imh0dHBzOi8vYXV0aDAuZ3JpdC5pby8iLCJzdWIiOiJnaXRodWJ8MTYyNzgwMSIsImF1ZCI6Imh0dHBzOi8vYXBpMi5ncml0LmlvIiwiaWF0IjoxNzE4NzI2MzUzLCJleHAiOjE3MTg4MTI3NTN9.eEU0bSldfdxuWpXAKfWAuJBqTMR5BAdnAEhFu-hVlI4";
        let auth_info = AuthInfo {
            access_token: jwt.to_string(),
            refresh_token: None,
        };

        match auth_info.get_user_name() {
            Ok(Some(username)) => assert_eq!(username, "morgante"),
            Ok(None) => panic!("Username not found"),
            Err(e) => panic!("Error occurred: {}", e),
        }
    }
}
