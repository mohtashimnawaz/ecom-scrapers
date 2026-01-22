use async_trait::async_trait;
use anyhow::{Result, anyhow};
use reqwest::Client;
use regex::Regex;
use serde_json::Value;
use crate::scraper_trait::PriceScraper;

pub struct AjioScraper {
    client: Client,
}

impl AjioScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .build()
            .expect("Failed to create HTTP client");
        
        AjioScraper { client }
    }
}

#[async_trait]
impl PriceScraper for AjioScraper {
    async fn get_price(&self, url: &str) -> Result<f64> {
        tracing::info!("Scraping Ajio URL: {}", url);
        
        let response = self.client
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.5")
            .send()
            .await?;
        
        let html = response.text().await?;
        
        // Look for window.__INITIAL_STATE__
        let re = Regex::new(r#"window\.__INITIAL_STATE__\s*=\s*(\{.*?\});"#)?;
        
        if let Some(captures) = re.captures(&html) {
            if let Some(json_str) = captures.get(1) {
                let data: Value = serde_json::from_str(json_str.as_str())?;
                
                // Navigate JSON structure to find price
                // Ajio typically stores price in: product.price.value or similar
                if let Some(product) = data.get("product") {
                    if let Some(price) = product["price"]["value"].as_f64() {
                        tracing::info!("Found Ajio price: ₹{}", price);
                        return Ok(price);
                    }
                    
                    // Alternative path
                    if let Some(price) = product["offerPrice"].as_f64() {
                        tracing::info!("Found Ajio offer price: ₹{}", price);
                        return Ok(price);
                    }
                }
            }
        }
        
        Err(anyhow!("Could not find price in Ajio HTML. Site structure may have changed."))
    }
    
    fn platform_name(&self) -> &'static str {
        "ajio"
    }
    
    fn can_handle(&self, url: &str) -> bool {
        url.contains("ajio.com")
    }
}
