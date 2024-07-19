use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct JwtToken {
    pub user_id: String,
    pub exp: usize,
}

impl JwtToken {
    pub fn get_key() -> String {
        "bach".to_string()
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(JwtToken::get_key().as_ref());
        jsonwebtoken::encode(&Header::default(), &self, &key).unwrap()
    }

    pub fn new(user_id: String) -> JwtToken {
        JwtToken { user_id, exp: 100 }
    }

    pub fn from_token(token: String) -> Result<Self, String> {
        let key = DecodingKey::from_secret(JwtToken::get_key().as_ref());
        let token_data = decode::<JwtToken>(
            &token,
            &key,
            &Validation::new(jsonwebtoken::Algorithm::HS256),
        );

        match token_data {
            Ok(token_data) => Ok(token_data.claims),
            Err(e) => Err(e.to_string()),
        }
    }
}
