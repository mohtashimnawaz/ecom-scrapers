use async_trait::async_trait;
use anyhow::{Result, anyhow};
use reqwest::Client;
use regex::Regex;
use serde_json::Value;
use crate::scraper_trait::PriceScraper;

pub struct MyntraScraper {
    client: Client,
}

impl MyntraScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .build()
            .expect("Failed to create HTTP client");
        
        MyntraScraper { client }
    }
}

#[async_trait]
impl PriceScraper for MyntraScraper {
    async fn get_price(&self, url: &str) -> Result<f64> {
        tracing::info!("Scraping Myntra URL: {}", url);
        
        let response = self.client
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.5")
            .send()
            .await?;
        
        let html = response.text().await?;
        
        // Primary: Look for window.__myntra_preloaded_state__ (2026 spec)
        let re_preloaded = Regex::new(r#"window\.__myntra_preloaded_state__\s*=\s*(\{[\s\S]*?\});"#)?;
        if let Some(captures) = re_preloaded.captures(&html) {
            if let Some(json_str) = captures.get(1) {
                if let Ok(data) = serde_json::from_str::<Value>(json_str.as_str()) {
                    // Navigate the preloaded state structure
                    if let Some(price) = data["pdpData"]["price"]["discounted"].as_f64() {
                        tracing::info!("Found Myntra price (preloaded_state): ₹{}", price);
                        return Ok(price);
                    }
                    if let Some(price) = data["pdpData"]["price"]["mrp"].as_f64() {
                        tracing::info!("Found Myntra MRP (preloaded_state): ₹{}", price);
                        return Ok(price);
                    }
                }
            }
        }
        
        // Fallback: Look for pdpData in script tags
        let re = Regex::new(r#"pdpData["\s:]+(\{.*?\})\s*[,;]"#)?;
        if let Some(captures) = re.captures(&html) {
            if let Some(json_str) = captures.get(1) {
                let data: Value = serde_json::from_str(json_str.as_str())?;
                
                if let Some(price) = data["price"]["discounted"].as_f64() {
                    tracing::info!("Found Myntra price (pdpData): ₹{}", price);
                    return Ok(price);
                }
                
                if let Some(price) = data["mrp"].as_f64() {
                    tracing::info!("Found Myntra MRP (pdpData): ₹{}", price);
                    return Ok(price);
                }
            }
        }
        
        Err(anyhow!("Could not find price in Myntra HTML. Site structure may have changed."))
    }
    
    fn platform_name(&self) -> &'static str {
        "myntra"
    }
    
    fn can_handle(&self, url: &str) -> bool {
        url.contains("myntra.com")
    }
}
