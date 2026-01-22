use std::time::Duration;
use tokio::time::interval;
use mongodb::bson::doc;
use chrono::Utc;
use crate::db::MongoDb;
use crate::scrapers::create_scraper;

pub async fn start_price_monitor(db: MongoDb) {
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

async fn check_all_alerts(db: MongoDb) -> anyhow::Result<()> {
    let collection = db.alerts_collection();
    
    // Find all active alerts
    let filter = doc! { "is_active": true };
    let mut cursor = collection.find(filter, None).await?;
    
    let mut alerts_checked = 0;
    let mut price_drops = 0;
    
    use futures::stream::StreamExt;
    
    while let Some(result) = cursor.next().await {
        let mut alert = result?;
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
                    alert.id.as_ref().map(|id| id.to_hex()).unwrap_or_default(),
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
                alert.last_price = Some(current_price);
                alert.last_checked = Utc::now();
                
                let update = doc! {
                    "$set": {
                        "last_price": current_price,
                        "last_checked": mongodb::bson::to_bson(&Utc::now())?
                    }
                };
                
                if let Some(id) = alert.id {
                    collection.update_one(
                        doc! { "_id": id },
                        update,
                        None
                    ).await?;
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
pub async fn trigger_manual_check(db: MongoDb) -> anyhow::Result<String> {
    check_all_alerts(db).await?;
    Ok("Price check completed".to_string())
}
