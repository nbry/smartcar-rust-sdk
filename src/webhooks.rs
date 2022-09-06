use hex;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

use crate::error::Error;

pub fn hash_challenge(amt: &str, challenge: &str) -> Result<String, Error> {
    let mut mac = HmacSha256::new_from_slice(challenge.as_bytes())?;
    mac.update(amt.as_bytes());
    let mac_bytes = mac.finalize().into_bytes();

    Ok(hex::encode(mac_bytes))
}

pub fn verify_payload(amt: &str, signature: &str, body: &str) -> Result<bool, Error> {
    Ok(hash_challenge(amt, body)? == signature.to_string())
}

#[test]
fn test_hash_challenge() {
    let amt = "abc123abc123";
    let body = "9c9c9c9c";
    let hex_encoding = hash_challenge(amt, body).unwrap();
    let verified_payload = verify_payload(amt, hex_encoding.as_str(), body).unwrap();

    assert!(verified_payload == true);
}
