use async_trait::async_trait;
use anyhow::Result;

/// Trait for platform-specific price scrapers
#[async_trait]
pub trait PriceScraper: Send + Sync {
    /// Extract the current price from a product URL
    async fn get_price(&self, url: &str) -> Result<f64>;
    
    /// Get the platform name
    fn platform_name(&self) -> &'static str;
    
    /// Validate if a URL belongs to this platform
    fn can_handle(&self, url: &str) -> bool;
}

/// Determine which scraper to use based on URL
pub fn detect_platform(url: &str) -> Option<&'static str> {
    if url.contains("myntra.com") {
        Some("myntra")
    } else if url.contains("flipkart.com") {
        Some("flipkart")
    } else if url.contains("ajio.com") {
        Some("ajio")
    } else if url.contains("tatacliq.com") {
        Some("tata_cliq")
    } else {
        None
    }
}
