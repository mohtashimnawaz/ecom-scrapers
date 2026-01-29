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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claims_creation() {
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        
        let claims = Claims::new(user_id, email.clone());
        
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
        assert!(claims.exp > claims.iat);
        assert_eq!(claims.exp - claims.iat, 24 * 3600); // 24 hours in seconds
    }

    #[test]
    fn test_token_generation_and_verification() {
        unsafe { std::env::set_var("JWT_SECRET", "test_secret_key_12345"); }
        
        let user_id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        
        // Generate token
        let token = generate_token(user_id, email.clone()).unwrap();
        assert!(!token.is_empty());
        
        // Verify token
        let claims = verify_token(&token).unwrap();
        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.email, email);
    }

    #[test]
    fn test_invalid_token_verification() {
        unsafe { std::env::set_var("JWT_SECRET", "test_secret_key_12345"); }
        
        let invalid_token = "invalid.jwt.token";
        let result = verify_token(invalid_token);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_token_with_wrong_secret() {
        unsafe { std::env::set_var("JWT_SECRET", "secret1"); }
        let user_id = Uuid::new_v4();
        let token = generate_token(user_id, "test@example.com".to_string()).unwrap();
        
        // Change secret
        unsafe { std::env::set_var("JWT_SECRET", "secret2"); }
        let result = verify_token(&token);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_password_hashing() {
        let password = "SecurePassword123!";
        
        let hashed = hash_password(password).unwrap();
        assert_ne!(hashed, password);
        assert!(hashed.starts_with("$2b$")); // bcrypt hash format
    }

    #[test]
    fn test_password_verification() {
        let password = "SecurePassword123!";
        let hashed = hash_password(password).unwrap();
        
        // Correct password
        let valid = verify_password(password, &hashed).unwrap();
        assert!(valid);
        
        // Wrong password
        let invalid = verify_password("WrongPassword", &hashed).unwrap();
        assert!(!invalid);
    }

    #[test]
    fn test_different_passwords_produce_different_hashes() {
        let password1 = "password1";
        let password2 = "password2";
        
        let hash1 = hash_password(password1).unwrap();
        let hash2 = hash_password(password2).unwrap();
        
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_same_password_produces_different_hashes() {
        // bcrypt uses random salt, so same password should produce different hashes
        let password = "SamePassword123";
        
        let hash1 = hash_password(password).unwrap();
        let hash2 = hash_password(password).unwrap();
        
        assert_ne!(hash1, hash2);
        
        // But both should verify correctly
        assert!(verify_password(password, &hash1).unwrap());
        assert!(verify_password(password, &hash2).unwrap());
    }
}
