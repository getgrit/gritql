use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64;
use base64::engine::Engine as _;
use grit_util::error::{GritPatternError, GritResult};

pub fn decode_to_string(input: &[u8]) -> GritResult<String> {
    let decoded = BASE64
        .decode(input)
        .map_err(|e| GritPatternError::new(e.to_string()))?;
    let raw = String::from_utf8(decoded)?;
    Ok(raw)
}

pub fn encode_from_bytes(input: &[u8]) -> GritResult<String> {
    let encoded = BASE64.encode(input);
    Ok(encoded)
}

pub fn encode_from_string(input: &str) -> GritResult<String> {
    let encoded = BASE64.encode(input.as_bytes());
    Ok(encoded)
}
