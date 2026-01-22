pub mod myntra;
pub mod flipkart;
pub mod ajio;
pub mod tata_cliq;

use crate::scraper_trait::PriceScraper;
use std::sync::Arc;

pub fn create_scraper(platform: &str) -> Option<Arc<dyn PriceScraper>> {
    match platform {
        "myntra" => Some(Arc::new(myntra::MyntraScraper::new())),
        "flipkart" => Some(Arc::new(flipkart::FlipkartScraper::new())),
        "ajio" => Some(Arc::new(ajio::AjioScraper::new())),
        "tata_cliq" => Some(Arc::new(tata_cliq::TataCliqScraper::new())),
        _ => None,
    }
}
