use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use marzano_util::base64::decode_to_string;

#[derive(Clone, Debug)]
pub struct AuthInfo {
    pub access_token: String,
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
        Self { access_token }
    }

    fn get_payload(&self) -> Result<AuthInfoPayload> {
        let basics = self.access_token.split('.').nth(1).ok_or(anyhow::anyhow!(
            "Invalid token format, expected 3 parts separated by '.'"
        ))?;
        let raw = decode_to_string(basics.as_bytes())?;

        let payload: AuthInfoPayload = serde_json::from_str(&raw)?;

        Ok(payload)
    }

    pub fn get_expiry(&self) -> Result<DateTime<Utc>> {
        let payload = self.get_payload()?;
        let expiry = DateTime::<Utc>::from_timestamp(payload.exp as i64, 0)
            .ok_or(anyhow::anyhow!("Invalid timestamp"))?;
        Ok(expiry)
    }

    pub fn get_user_name(&self) -> Result<Option<String>> {
        let payload = self.get_payload()?;
        Ok(payload.hasura_claims.and_then(|c| c.nickname))
    }

    pub fn get_user_id(&self) -> Result<String> {
        let payload = self.get_payload()?;
        Ok(payload.sub)
    }

    pub fn is_expired(&self) -> Result<bool> {
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
        let jwt = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJodHRwczovL2F1dGgwLmdyaXQuaW8vIiwic3ViIjoiZ2l0aHVifDE2Mjc4MDEiLCJhdWQiOiJodHRwczovL2FwaTIuZ3JpdC5pbyIsImlhdCI6MTcxODcyNjM1MywiZXhwIjoxNzE4ODEyNzUzLCJzY29wZSI6Im9mZmxpbmVfYWNjZXNzIiwiaHR0cHM6Ly9oYXN1cmEuaW8vand0L2NsYWltcyI6eyJ4LWhhc3VyYS1kZWZhdWx0LXJvbGUiOiJ1c2VyIiwieC1oYXN1cmEtYWxsb3dlZC1yb2xlcyI6WyJ1c2VyIl0sIngtaGFzdXJhLXVzZXItaWQiOiJnaXRodWJ8MTYyNzgwMSIsIngtaGFzdXJhLXJhdy1uaWNrbmFtZSI6Im5hbmN5IiwieC1oYXN1cmEtdXNlci10ZW5hbnQiOiJnaXRodWIiLCJ4LWhhc3VyYS1hdXRoLXByb3ZpZGVyIjoiZ2l0aHViIiwieC1oYXN1cmEtdXNlci1uaWNrbmFtZSI6ImdpdGh1YnxuYW5jeSJ9fQ.UNiKbUgbITr4aKhZuwEwXzmjeH6kyHxQjQiL4YODGWY";
        let auth_info = AuthInfo {
            access_token: jwt.to_string(),
        };

        match auth_info.get_user_name() {
            Ok(Some(username)) => assert_eq!(username, "nancy"),
            Ok(None) => panic!("Username not found"),
            Err(e) => panic!("Error occurred: {}", e),
        }
    }
}
