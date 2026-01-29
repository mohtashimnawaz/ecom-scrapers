use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct PriceAlert {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    pub url: String,
    pub target_price: f64,
    pub last_price: Option<f64>,
    pub user_email: String,
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
