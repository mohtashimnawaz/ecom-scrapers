use anyhow::Result;
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // user_id
    pub email: String,
    pub exp: i64,     // Expiration timestamp
    pub iat: i64,     // Issued at
}

impl Claims {
    pub fn new(user_id: Uuid, email: String) -> Self {
        let now = Utc::now();
        let expiry = now + Duration::hours(24); // Token valid for 24 hours
        
        Claims {
            sub: user_id.to_string(),
            email,
            exp: expiry.timestamp(),
            iat: now.timestamp(),
        }
    }
}

// JWT token generator
pub fn generate_token(user_id: Uuid, email: String) -> Result<String> {
    let claims = Claims::new(user_id, email);
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_key_change_in_production".to_string());
    
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    
    Ok(token)
}

// JWT token validator
pub fn verify_token(token: &str) -> Result<Claims> {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret_key_change_in_production".to_string());
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    
    Ok(token_data.claims)
}

// Axum extractor for authenticated requests
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
    pub email: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Missing or invalid Authorization header".to_string(),
                )
            })?;

        // Verify token
        let claims = verify_token(bearer.token()).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                format!("Invalid token: {}", e),
            )
        })?;

        // Parse user_id
        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "Invalid user ID in token".to_string(),
            )
        })?;

        Ok(AuthUser {
            user_id,
            email: claims.email,
        })
    }
}

// Password hashing utilities
pub fn hash_password(password: &str) -> Result<String> {
    let hashed = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let valid = bcrypt::verify(password, hash)?;
    Ok(valid)
}
