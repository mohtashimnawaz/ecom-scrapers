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
        
        // Look for window.__myx or pdpData in script tags
        let re = Regex::new(r#"pdpData["\s:]+(\{.*?\})\s*[,;]"#)?;
        
        if let Some(captures) = re.captures(&html) {
            if let Some(json_str) = captures.get(1) {
                let data: Value = serde_json::from_str(json_str.as_str())?;
                
                // Try multiple paths where price might be
                if let Some(price) = data["price"]["discounted"].as_f64() {
                    tracing::info!("Found Myntra price: ₹{}", price);
                    return Ok(price);
                }
                
                if let Some(price) = data["mrp"].as_f64() {
                    tracing::info!("Found Myntra MRP: ₹{}", price);
                    return Ok(price);
                }
            }
        }
        
        // Fallback: Look for __PRELOADED_STATE__
        let re2 = Regex::new(r#"window\.__myx\s*=\s*(\{.*?\});"#)?;
        if let Some(captures) = re2.captures(&html) {
            if let Some(json_str) = captures.get(1) {
                let data: Value = serde_json::from_str(json_str.as_str())?;
                
                if let Some(price) = data["pdpData"]["price"]["discounted"].as_f64() {
                    tracing::info!("Found Myntra price (alternative): ₹{}", price);
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
