mod models;
mod db;
mod scraper_trait;
mod scrapers;
mod worker;
mod api;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "clothing_price_tracker=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Get MongoDB connection string from environment
    let mongodb_uri = std::env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let db_name = std::env::var("DB_NAME")
        .unwrap_or_else(|_| "price_tracker".to_string());
    
    tracing::info!("Connecting to MongoDB...");
    let db = db::MongoDb::new(&mongodb_uri, &db_name).await?;
    
    // Start background worker
    let worker_db = db.clone();
    tokio::spawn(async move {
        worker::start_price_monitor(worker_db).await;
    });
    
    // Create API router
    let app = api::create_router(db);
    
    // Server address
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    tracing::info!("🚀 Server starting on http://{}", addr);
    tracing::info!("🎨 Frontend available at http://{}/app", addr);
    tracing::info!("📊 Monitoring prices every 6 hours");
    tracing::info!("📝 API Endpoints:");
    tracing::info!("  GET  /           - Health check");
    tracing::info!("  POST /alerts     - Create price alert");
    tracing::info!("  GET  /alerts     - List all alerts");
    tracing::info!("  DELETE /alerts/:id - Delete alert");
    tracing::info!("  POST /alerts/check - Manually trigger price check");
    
    // Create TCP listener
    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Start server
    axum::serve(listener, app).await?;
    
    Ok(())
}
