use async_trait::async_trait;
use anyhow::{Result, anyhow};
use reqwest::Client;
use scraper::{Html, Selector};
use crate::scraper_trait::PriceScraper;

pub struct FlipkartScraper {
    client: Client,
}

impl FlipkartScraper {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
            .build()
            .expect("Failed to create HTTP client");
        
        FlipkartScraper { client }
    }
    
    fn parse_price(&self, price_str: &str) -> Result<f64> {
        let cleaned = price_str
            .replace('₹', "")
            .replace(',', "")
            .trim()
            .to_string();
        
        cleaned.parse::<f64>()
            .map_err(|e| anyhow!("Failed to parse price '{}': {}", price_str, e))
    }
}

#[async_trait]
impl PriceScraper for FlipkartScraper {
    async fn get_price(&self, url: &str) -> Result<f64> {
        tracing::info!("Scraping Flipkart URL: {}", url);
        
        let response = self.client
            .get(url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.5")
            .send()
            .await?;
        
        let html = response.text().await?;
        let document = Html::parse_document(&html);
        
        // Try multiple selectors as Flipkart changes them frequently
        let selectors = vec![
            ".Nx9bqj",  // Current price selector (2026)
            "._30jeq3", // Alternative
            "._16Jk6d", // Another alternative
            ".CEmiEU",  // Older selector
        ];
        
        for selector_str in selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(element) = document.select(&selector).next() {
                    let price_text = element.text().collect::<String>();
                    if let Ok(price) = self.parse_price(&price_text) {
                        tracing::info!("Found Flipkart price: ₹{}", price);
                        return Ok(price);
                    }
                }
            }
        }
        
        Err(anyhow!("Could not find price in Flipkart HTML. Site structure may have changed."))
    }
    
    fn platform_name(&self) -> &'static str {
        "flipkart"
    }
    
    fn can_handle(&self, url: &str) -> bool {
        url.contains("flipkart.com")
    }
}
