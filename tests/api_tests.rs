use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use serde_json::json;
use clothing_price_tracker::api::create_router;
use clothing_price_tracker::db::Database;
use sqlx::PgPool;
use serial_test::serial;

// Helper to create test database connection
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost/price_tracker_test".to_string());
    
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database");
    
    // Create tables directly (simpler than migrations for tests)
    let schema = r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        );

        CREATE TABLE IF NOT EXISTS price_alerts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID REFERENCES users(id) ON DELETE CASCADE,
            url TEXT NOT NULL,
            current_price DECIMAL(10,2),
            target_price DECIMAL(10,2) NOT NULL,
            platform VARCHAR(50),
            product_name TEXT,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            last_checked TIMESTAMPTZ,
            is_active BOOLEAN DEFAULT true
        );

        CREATE TABLE IF NOT EXISTS price_history (
            id SERIAL PRIMARY KEY,
            alert_id UUID REFERENCES price_alerts(id) ON DELETE CASCADE,
            price DECIMAL(10,2) NOT NULL,
            checked_at TIMESTAMPTZ DEFAULT NOW()
        );
    "#;
    
    sqlx::query(schema)
        .execute(&pool)
        .await
        .expect("Failed to create schema");
    
    pool
}

// Helper to clean test database
async fn cleanup_test_db(pool: &PgPool) {
    sqlx::query("DELETE FROM price_alerts")
        .execute(pool)
        .await
        .ok();
    
    sqlx::query("DELETE FROM users")
        .execute(pool)
        .await
        .ok();
}

#[tokio::test]
#[serial]
async fn test_health_check() {
    let pool = setup_test_db().await;
    let db = Database::new(pool.clone());
    let app = create_router(db);
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_signup_and_login() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_integration_tests");
    
    let pool = setup_test_db().await;
    let db = Database::new(pool.clone());
    
    cleanup_test_db(&pool).await;
    
    let app = create_router(db);
    
    // Test signup
    let signup_request = json!({
        "email": "testuser@example.com",
        "password": "SecurePassword123!"
    });
    
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/signup")
                .header("content-type", "application/json")
                .body(Body::from(signup_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::CREATED);
    
    // Test login
    let login_request = json!({
        "email": "testuser@example.com",
        "password": "SecurePassword123!"
    });
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_login_with_wrong_password() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_integration_tests");
    
    let pool = setup_test_db().await;
    let db = Database::new(pool.clone());
    
    cleanup_test_db(&pool).await;
    
    let app = create_router(db);
    
    // Create user
    let signup_request = json!({
        "email": "testuser2@example.com",
        "password": "CorrectPassword123!"
    });
    
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/signup")
                .header("content-type", "application/json")
                .body(Body::from(signup_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Try login with wrong password
    let login_request = json!({
        "email": "testuser2@example.com",
        "password": "WrongPassword123!"
    });
    
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_protected_route_without_auth() {
    let pool = setup_test_db().await;
    let db = Database::new(pool.clone());
    let app = create_router(db);
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/auth/me")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_create_and_list_alerts() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_integration_tests");
    
    let pool = setup_test_db().await;
    let db = Database::new(pool.clone());
    
    cleanup_test_db(&pool).await;
    
    let app = create_router(db);
    
    // Signup and login to get token
    let signup_request = json!({
        "email": "alertuser@example.com",
        "password": "Password123!"
    });
    
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/signup")
                .header("content-type", "application/json")
                .body(Body::from(signup_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let login_request = json!({
        "email": "alertuser@example.com",
        "password": "Password123!"
    });
    
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let body_bytes = axum::body::to_bytes(login_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let login_data: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let token = login_data["token"].as_str().unwrap();
    
    // Create alert
    let alert_request = json!({
        "url": "https://www.myntra.com/shirts/nike/12345",
        "target_price": 999.0
    });
    
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/alerts")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(alert_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::CREATED);
    
    // List alerts
    let response = app
        .oneshot(
            Request::builder()
                .uri("/alerts")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::OK);
    
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let alerts: Vec<serde_json::Value> = serde_json::from_slice(&body_bytes).unwrap();
    
    assert_eq!(alerts.len(), 1);
    assert_eq!(alerts[0]["url"], "https://www.myntra.com/shirts/nike/12345");
    assert_eq!(alerts[0]["target_price"], 999.0);
    
    cleanup_test_db(&pool).await;
}

#[tokio::test]
#[serial]
async fn test_delete_alert() {
    std::env::set_var("JWT_SECRET", "test_secret_key_for_integration_tests");
    
    let pool = setup_test_db().await;
    let db = Database::new(pool.clone());
    
    cleanup_test_db(&pool).await;
    
    let app = create_router(db);
    
    // Setup: Create user and alert
    let signup_request = json!({
        "email": "deleteuser@example.com",
        "password": "Password123!"
    });
    
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/signup")
                .header("content-type", "application/json")
                .body(Body::from(signup_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let login_request = json!({
        "email": "deleteuser@example.com",
        "password": "Password123!"
    });
    
    let login_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(login_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let body_bytes = axum::body::to_bytes(login_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let login_data: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let token = login_data["token"].as_str().unwrap();
    
    let alert_request = json!({
        "url": "https://www.flipkart.com/product/abc",
        "target_price": 1299.0
    });
    
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/alerts")
                .header("content-type", "application/json")
                .header("authorization", format!("Bearer {}", token))
                .body(Body::from(alert_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let body_bytes = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let alert_data: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let alert_id = alert_data["id"].as_str().unwrap();
    
    // Delete alert
    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(&format!("/alerts/{}", alert_id))
                .header("authorization", format!("Bearer {}", token))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    
    cleanup_test_db(&pool).await;
}
