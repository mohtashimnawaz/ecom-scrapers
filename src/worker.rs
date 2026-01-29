use std::time::Duration;
use tokio::time::interval;
use crate::db::Database;
use crate::scrapers::create_scraper;

pub async fn start_price_monitor(db: Database) {
    tracing::info!("Starting background price monitoring worker (6-hour interval)");
    
    let mut ticker = interval(Duration::from_secs(6 * 60 * 60)); // 6 hours
    
    loop {
        ticker.tick().await;
        
        tracing::info!("Running scheduled price check...");
        
        if let Err(e) = check_all_alerts(db.clone()).await {
            tracing::error!("Error during price check: {}", e);
        }
    }
}

async fn check_all_alerts(db: Database) -> anyhow::Result<()> {
    let alerts = db.get_all_active_alerts().await?;
    
    let mut alerts_checked = 0;
    let mut price_drops = 0;
    
    for alert in alerts {
        alerts_checked += 1;
        
        // Get the appropriate scraper
        let scraper = match create_scraper(&alert.platform) {
            Some(s) => s,
            None => {
                tracing::warn!("Unknown platform: {}", alert.platform);
                continue;
            }
        };
        
        // Scrape current price
        match scraper.get_price(&alert.url).await {
            Ok(current_price) => {
                tracing::info!(
                    "Alert {}: Current=₹{}, Target=₹{}, Last=₹{:?}",
                    alert.id.map(|id| id.to_string()).unwrap_or_default(),
                    current_price,
                    alert.target_price,
                    alert.last_price
                );
                
                // Check if price dropped below target
                if current_price <= alert.target_price {
                    tracing::warn!(
                        "🚨 ALARM! Price drop detected for {}: ₹{} <= ₹{} (Target)",
                        alert.user_email,
                        current_price,
                        alert.target_price
                    );
                    price_drops += 1;
                    
                    // TODO: Send email notification here
                    // send_email(&alert.user_email, &alert.url, current_price, alert.target_price).await?;
                }
                
                // Update alert with new price
                if let Some(id) = alert.id {
                    db.update_alert_price(id, current_price).await?;
                }
            }
            Err(e) => {
                tracing::error!("Failed to scrape {}: {}", alert.url, e);
            }
        }
        
        // Small delay to avoid rate limiting
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    
    tracing::info!(
        "Price check complete. Checked: {}, Drops detected: {}",
        alerts_checked,
        price_drops
    );
    
    Ok(())
}

/// Manual trigger for testing (can be exposed via API)
pub async fn trigger_manual_check(db: Database) -> anyhow::Result<String> {
    check_all_alerts(db).await?;
    Ok("Price check completed".to_string())
}
