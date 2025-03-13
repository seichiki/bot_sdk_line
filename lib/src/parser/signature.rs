use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;

pub fn validate_signature(channel_secret: &str, signature: &str, body: &str) -> bool {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(channel_secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(body.as_bytes());
    let mut buf = String::new();
    general_purpose::STANDARD.encode_string(mac.finalize().into_bytes(), &mut buf);
    buf == signature
}