use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// User model for authentication
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct PriceAlert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub url: String,
    pub target_price: f64,
    pub last_price: Option<f64>,
    pub user_email: String,
    pub user_id: Option<Uuid>,
    pub platform: String, // myntra, flipkart, ajio, tata_cliq
    pub created_at: DateTime<Utc>,
    pub last_checked: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAlertRequest {
    pub url: String,
    pub target_price: f64,
    pub user_email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertResponse {
    pub id: String,
    pub url: String,
    pub target_price: f64,
    pub last_price: Option<f64>,
    pub user_email: String,
    pub platform: String,
}

impl From<PriceAlert> for AlertResponse {
    fn from(alert: PriceAlert) -> Self {
        AlertResponse {
            id: alert.id.map(|id| id.to_string()).unwrap_or_default(),
            url: alert.url,
            target_price: alert.target_price,
            last_price: alert.last_price,
            user_email: alert.user_email,
            platform: alert.platform,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct PriceHistory {
    pub id: Uuid,
    pub alert_id: Uuid,
    pub price: f64,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct PriceStats {
    pub lowest_price: Option<f64>,
    pub highest_price: Option<f64>,
    pub average_price: Option<f64>,
    pub data_points: Option<i64>,
}

// Auth request/response models
#[derive(Debug, Deserialize)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserResponse,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

