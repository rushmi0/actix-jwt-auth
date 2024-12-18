use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Result};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone)]
pub struct UserClaims {
    pub id: u32,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Role {
    Admin,
    BaseUser,
}


const SECRET_KEY: &[u8] = b"ffffffff";

pub fn create_jwt(user_claims: &UserClaims) -> Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + 3600;

    let claims = Claims {
        sub: serde_json::to_string(user_claims).expect("Failed to serialize user claims"),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}

pub fn verify_jwt(token: &str) -> Result<UserClaims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )?;
    let user_claims: UserClaims = serde_json::from_str(&token_data.claims.sub)
        .expect("Failed to deserialize user claims");
    Ok(user_claims)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    pub sub: String,
    pub exp: usize,
}
