use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: Uuid,
    pub iat: i64,
    pub exp: i64,
}

pub fn issue(user_id: Uuid, secret: &[u8], ttl: Duration) -> Result<String> {
    let now = Utc::now();
    let claims = Claims {
        sub: user_id,
        iat: now.timestamp(),
        exp: (now + ttl).timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn validate(token: &str, secret: &[u8]) -> Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

#[cfg(test)]
mod tests {
    use chrono::Duration;
    use uuid::Uuid;

    #[test]
    fn jwt_roundtrip() {
        let secret = "secret".as_bytes();
        let id = Uuid::new_v4();
        let token = super::issue(id, secret, Duration::minutes(30)).unwrap();
        let claims = super::validate(&token, secret).unwrap();
        assert_eq!(claims.sub, id);
    }
}
