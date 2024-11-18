use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const SECRET_KEY: &[u8] = b"secret";

pub fn create_jwt(user_id: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(3600))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY)).unwrap()
}

pub fn validate_jwt(token: &str) -> bool {
    let validation = Validation::default();
    decode::<Claims>(token, &DecodingKey::from_secret(SECRET_KEY), &validation).is_ok()
}