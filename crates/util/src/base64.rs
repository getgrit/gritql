use anyhow::Result;
use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;
use base64::engine::Engine as _;

pub fn decode_to_string(input: &[u8]) -> Result<String> {
    let decoded = BASE64.decode(input)?;
    let raw = String::from_utf8(decoded)?;
    Ok(raw)
}

pub fn encode_from_bytes(input: &[u8]) -> Result<String> {
    let encoded = BASE64.encode(input);
    Ok(encoded)
}

pub fn encode_from_string(input: &str) -> Result<String> {
    let encoded = BASE64.encode(input.as_bytes());
    Ok(encoded)
}
