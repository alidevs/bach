use axum::{
    async_trait,
    body::Body,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::Response,
};
use jsonwebtoken::{decode, errors::Error, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims {
    fn get_key() -> String {
        "bach".to_string()
    }

    fn get_exp() -> usize {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        let exp = now + Duration::from_secs(3600);

        exp.as_secs() as usize
    }

    pub fn encode(self) -> String {
        let key = EncodingKey::from_secret(Claims::get_key().as_ref());
        jsonwebtoken::encode(&Header::default(), &self, &key).unwrap()
    }

    pub fn new(user_id: String) -> Claims {
        Claims {
            sub: user_id,
            exp: Claims::get_exp(),
        }
    }

    fn from_token(token: String) -> Result<Self, Error> {
        let decoding_key = DecodingKey::from_secret(Claims::get_key().as_ref());
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        let token_data = decode::<Claims>(&token, &decoding_key, &validation)?;

        Ok(token_data.claims)
    }

    pub fn into_response(self) -> Response<Body> {
        return Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&self).unwrap()))
            .unwrap();
    }
}

#[async_trait]
impl<AppState> FromRequestParts<AppState> for Claims {
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .ok_or((StatusCode::UNAUTHORIZED, "missing Authorization header"))?
            .to_str()
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    "invalid Authorization header format",
                )
            })?;

        if !token.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "invalid token format"));
        }

        let token = &token[7..];

        let claims = Claims::from_token(token.to_string()).map_err(|e| {
            error!("Token decoding error: {:?}", e);
            (StatusCode::UNAUTHORIZED, "invalid token")
        })?;

        tracing::info!("claims: {:?}", claims);
        tracing::info!("token: {:?}", token);

        Ok(claims)
    }
}
