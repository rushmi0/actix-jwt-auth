use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Result};
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

const SECRET_KEY: &[u8] = b"ffffffff";

pub fn create_jwt(sub: &str) -> Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600;

    let claims = Claims {
        sub: sub.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}

pub fn verify_jwt(token: &str) -> Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}


