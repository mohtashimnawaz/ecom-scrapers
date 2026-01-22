use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post, delete},
    Router,
};
use mongodb::bson::{doc, oid::ObjectId};
use chrono::Utc;
use serde_json::json;

use crate::db::MongoDb;
use crate::models::{CreateAlertRequest, PriceAlert, AlertResponse};
use crate::scraper_trait::detect_platform;
use crate::worker::trigger_manual_check;

#[derive(Clone)]
pub struct AppState {
    pub db: MongoDb,
}

pub fn create_router(db: MongoDb) -> Router {
    let state = AppState { db };
    
    Router::new()
        .route("/", get(health_check))
        .route("/alerts", post(create_alert))
        .route("/alerts", get(list_alerts))
        .route("/alerts/:id", delete(delete_alert))
        .route("/alerts/check", post(manual_price_check))
        .with_state(state)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "clothing-price-tracker",
        "version": "0.1.0"
    }))
}

async fn create_alert(
    State(state): State<AppState>,
    Json(payload): Json<CreateAlertRequest>,
) -> Result<(StatusCode, Json<AlertResponse>), (StatusCode, String)> {
    // Detect platform from URL
    let platform = detect_platform(&payload.url)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                "Unsupported platform. Supported: Myntra, Flipkart, Ajio, Tata Cliq".to_string(),
            )
        })?;
    
    // Validate target price
    if payload.target_price <= 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Target price must be greater than 0".to_string(),
        ));
    }
    
    // Create alert document
    let alert = PriceAlert {
        id: None,
        url: payload.url,
        target_price: payload.target_price,
        last_price: None,
        user_email: payload.user_email,
        platform: platform.to_string(),
        created_at: Utc::now(),
        last_checked: Utc::now(),
        is_active: true,
    };
    
    // Insert into database
    let collection = state.db.alerts_collection();
    let result = collection
        .insert_one(&alert, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let mut created_alert = alert;
    created_alert.id = Some(result.inserted_id.as_object_id().unwrap());
    
    Ok((StatusCode::CREATED, Json(created_alert.into())))
}

async fn list_alerts(
    State(state): State<AppState>,
) -> Result<Json<Vec<AlertResponse>>, (StatusCode, String)> {
    let collection = state.db.alerts_collection();
    
    let filter = doc! { "is_active": true };
    let mut cursor = collection
        .find(filter, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    use futures::stream::StreamExt;
    let mut alerts = Vec::new();
    
    while let Some(result) = cursor.next().await {
        let alert = result.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        alerts.push(alert.into());
    }
    
    Ok(Json(alerts))
}

async fn delete_alert(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let object_id = ObjectId::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid alert ID".to_string()))?;
    
    let collection = state.db.alerts_collection();
    
    // Soft delete by setting is_active to false
    let update = doc! { "$set": { "is_active": false } };
    let result = collection
        .update_one(doc! { "_id": object_id }, update, None)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    if result.matched_count == 0 {
        return Err((StatusCode::NOT_FOUND, "Alert not found".to_string()));
    }
    
    Ok(StatusCode::NO_CONTENT)
}

async fn manual_price_check(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    trigger_manual_check(state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(json!({ "message": "Price check triggered successfully" })))
}
