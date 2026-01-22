use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PriceAlert {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub url: String,
    pub target_price: f64,
    pub last_price: Option<f64>,
    pub user_email: String,
    pub platform: String, // myntra, flipkart, ajio, tata_cliq
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
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
            id: alert.id.map(|id| id.to_hex()).unwrap_or_default(),
            url: alert.url,
            target_price: alert.target_price,
            last_price: alert.last_price,
            user_email: alert.user_email,
            platform: alert.platform,
        }
    }
}
