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
            ".Nx9W0j",  // Current price selector (2026 spec)
            ".Nx9bqj",  // Alternative
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_flipkart_can_handle() {
        let scraper = FlipkartScraper::new();
        
        assert!(scraper.can_handle("https://www.flipkart.com/product/p/abc123"));
        assert!(scraper.can_handle("https://flipkart.com/item"));
        assert!(!scraper.can_handle("https://www.myntra.com/product"));
        assert!(!scraper.can_handle("https://www.ajio.com/product"));
    }

    #[tokio::test]
    async fn test_flipkart_platform_name() {
        let scraper = FlipkartScraper::new();
        assert_eq!(scraper.platform_name(), "flipkart");
    }

    #[tokio::test]
    async fn test_parse_price() {
        let scraper = FlipkartScraper::new();
        
        assert_eq!(scraper.parse_price("₹1,299").unwrap(), 1299.0);
        assert_eq!(scraper.parse_price("₹999").unwrap(), 999.0);
        assert_eq!(scraper.parse_price("1,999").unwrap(), 1999.0);
        assert_eq!(scraper.parse_price(" ₹2,500 ").unwrap(), 2500.0);
    }

    #[tokio::test]
    async fn test_flipkart_price_extraction() {
        let mut server = Server::new_async().await;
        
        let mock_html = r#"
            <!DOCTYPE html>
            <html>
            <body>
                <div class="Nx9W0j">₹1,499</div>
            </body>
            </html>
        "#;
        
        let _m = server.mock("GET", "/product/123")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body(mock_html)
            .create_async()
            .await;
        
        let scraper = FlipkartScraper::new();
        let url = format!("{}/product/123", server.url());
        let price = scraper.get_price(&url).await.unwrap();
        
        assert_eq!(price, 1499.0);
    }

    #[tokio::test]
    async fn test_flipkart_alternative_selector() {
        let mut server = Server::new_async().await;
        
        let mock_html = r#"
            <!DOCTYPE html>
            <html>
            <body>
                <div class="_30jeq3">₹2,999</div>
            </body>
            </html>
        "#;
        
        let _m = server.mock("GET", "/product/456")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body(mock_html)
            .create_async()
            .await;
        
        let scraper = FlipkartScraper::new();
        let url = format!("{}/product/456", server.url());
        let price = scraper.get_price(&url).await.unwrap();
        
        assert_eq!(price, 2999.0);
    }

    #[tokio::test]
    async fn test_flipkart_price_not_found() {
        let mut server = Server::new_async().await;
        
        let mock_html = r#"
            <!DOCTYPE html>
            <html>
            <body><p>No price here</p></body>
            </html>
        "#;
        
        let _m = server.mock("GET", "/product/invalid")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body(mock_html)
            .create_async()
            .await;
        
        let scraper = FlipkartScraper::new();
        let url = format!("{}/product/invalid", server.url());
        let result = scraper.get_price(&url).await;
        
        assert!(result.is_err());
    }
}
