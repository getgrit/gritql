use anyhow::Result;
use chrono::DateTime;
use chrono::Utc;
use marzano_util::base64::decode_to_string;

#[derive(Clone, Debug)]
pub struct AuthInfo {
    pub access_token: String,
}

#[derive(serde::Deserialize, Debug)]
struct AuthInfoPayload {
    exp: u64,
    sub: String,
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
