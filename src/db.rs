use anyhow::Result;
use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::models::{PriceAlert, PriceHistory, PriceStats};
use chrono::Utc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;
        
        // Run migrations to create tables if they don't exist
        Self::create_tables(&pool).await?;
        
        tracing::info!("Successfully connected to Supabase PostgreSQL");
        
        Ok(Database { pool })
    }
    
    async fn create_tables(pool: &PgPool) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS price_alerts (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                url TEXT NOT NULL,
                target_price DOUBLE PRECISION NOT NULL,
                last_price DOUBLE PRECISION,
                user_email TEXT NOT NULL,
                platform TEXT NOT NULL,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                last_checked TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                is_active BOOLEAN NOT NULL DEFAULT TRUE
            )
            "#
        )
        .execute(pool)
        .await?;
        
        // Create index on is_active for faster queries
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_is_active ON price_alerts(is_active)")
            .execute(pool)
            .await?;
        
        // Create price_history table for tracking price changes over time
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS price_history (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                alert_id UUID NOT NULL REFERENCES price_alerts(id) ON DELETE CASCADE,
                price DOUBLE PRECISION NOT NULL,
                checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            )
            "#
        )
        .execute(pool)
        .await?;
        
        // Create index on alert_id for faster lookups
        sqlx::query("CREATE INDEX IF NOT EXISTS idx_alert_id ON price_history(alert_id)")
            .execute(pool)
            .await?;
        
        tracing::info!("Database tables verified/created");
        Ok(())
    }
    
    pub async fn create_alert(&self, alert: &PriceAlert) -> Result<PriceAlert> {
        let result = sqlx::query_as::<_, PriceAlert>(
            r#"
            INSERT INTO price_alerts (url, target_price, last_price, user_email, platform, created_at, last_checked, is_active)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#
        )
        .bind(&alert.url)
        .bind(alert.target_price)
        .bind(alert.last_price)
        .bind(&alert.user_email)
        .bind(&alert.platform)
        .bind(alert.created_at)
        .bind(alert.last_checked)
        .bind(alert.is_active)
        .fetch_one(&self.pool)
        .await?;
        
        Ok(result)
    }
    
    pub async fn get_all_active_alerts(&self) -> Result<Vec<PriceAlert>> {
        let alerts = sqlx::query_as::<_, PriceAlert>(
            "SELECT * FROM price_alerts WHERE is_active = TRUE ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(alerts)
    }
    
    pub async fn update_alert_price(&self, id: Uuid, last_price: f64) -> Result<()> {
        sqlx::query(
            "UPDATE price_alerts SET last_price = $1, last_checked = $2 WHERE id = $3"
        )
        .bind(last_price)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn delete_alert(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE price_alerts SET is_active = FALSE WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        
        Ok(())
    }
    
    // Save price snapshot to history
    pub async fn save_price_snapshot(&self, alert_id: Uuid, price: f64) -> Result<()> {
        sqlx::query(
            "INSERT INTO price_history (alert_id, price, checked_at) VALUES ($1, $2, $3)"
        )
        .bind(alert_id)
        .bind(price)
        .bind(Utc::now())
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // Get price history for an alert
    pub async fn get_price_history(&self, alert_id: Uuid, limit: i64) -> Result<Vec<PriceHistory>> {
        let history = sqlx::query_as::<_, PriceHistory>(
            "SELECT * FROM price_history WHERE alert_id = $1 ORDER BY checked_at DESC LIMIT $2"
        )
        .bind(alert_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;
        
        Ok(history)
    }
    
    // Get price statistics for an alert
    pub async fn get_price_stats(&self, alert_id: Uuid) -> Result<Option<PriceStats>> {
        let stats = sqlx::query_as::<_, PriceStats>(
            r#"
            SELECT 
                MIN(price) as lowest_price,
                MAX(price) as highest_price,
                AVG(price) as average_price,
                COUNT(*) as data_points
            FROM price_history 
            WHERE alert_id = $1
            "#
        )
        .bind(alert_id)
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(stats)
    }
}
