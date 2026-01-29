use axum::{
    extract::{Path, State},
    http::{StatusCode, header, Method},
    response::Json,
    routing::{get, post, delete},
    Router,
};
use chrono::Utc;
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use uuid::Uuid;

use crate::db::Database;
use crate::models::{CreateAlertRequest, PriceAlert, AlertResponse};
use crate::email::EmailService;
use crate::scraper_trait::detect_platform;
use crate::worker::trigger_manual_check;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

pub fn create_router(db: Database) -> Router {
    let state = AppState { db };
    
    // CORS configuration
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE]);
    
    // API routes
    let api_routes = Router::new()
        .route("/", get(health_check))
        .route("/alerts", post(create_alert))
        .route("/alerts", get(list_alerts))
        .route("/alerts/:id", delete(delete_alert))
        .route("/alerts/:id/history", get(get_price_history))
        .route("/alerts/:id/stats", get(get_price_stats))
        .route("/email/test", post(test_email))
        .route("/alerts/check", post(manual_price_check))
        .with_state(state)
        .layer(cors);
    
    // Serve static frontend files
    let frontend_service = ServeDir::new("frontend")
        .append_index_html_on_directories(true);
    
    // Combine routes
    Router::new()
        .nest_service("/app", frontend_service)
        .merge(api_routes)
}

async fn health_check() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "clothing-price-tracker",
        "version": "0.1.0",
        "database": "supabase"
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
    let created_alert = state.db
        .create_alert(&alert)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok((StatusCode::CREATED, Json(created_alert.into())))
}

async fn list_alerts(
    State(state): State<AppState>,
) -> Result<Json<Vec<AlertResponse>>, (StatusCode, String)> {
    let alerts = state.db
        .get_all_active_alerts()
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    let responses: Vec<AlertResponse> = alerts.into_iter().map(|a| a.into()).collect();
    
    Ok(Json(responses))
}

async fn delete_alert(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    let uuid = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid alert ID".to_string()))?;
    
    state.db
        .delete_alert(uuid)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
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

async fn test_email(
    State(_state): State<AppState>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let to_email = payload["email"]
        .as_str()
        .ok_or_else(|| (StatusCode::BAD_REQUEST, "email field required".to_string()))?;
    
    let email_service = EmailService::from_env()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Email not configured: {}", e)))?;
    
    email_service.send_test_email(to_email)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to send email: {}", e)))?;
    
    Ok(Json(json!({ 
        "message": format!("Test email sent to {}", to_email),
        "status": "success"
    })))
}

async fn get_price_history(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let alert_id = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID".to_string()))?;
    
    // Get last 30 price checks (default)
    let history = state.db.get_price_history(alert_id, 30)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(json!({
        "alert_id": id,
        "history": history,
        "count": history.len()
    })))
}

async fn get_price_stats(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let alert_id = Uuid::parse_str(&id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID".to_string()))?;
    
    let stats = state.db.get_price_stats(alert_id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    match stats {
        Some(stats) => Ok(Json(json!({
            "alert_id": id,
            "lowest_price": stats.lowest_price,
            "highest_price": stats.highest_price,
            "average_price": stats.average_price,
            "data_points": stats.data_points
        }))),
        None => Ok(Json(json!({
            "alert_id": id,
            "message": "No price history available yet"
        })))
    }
}
