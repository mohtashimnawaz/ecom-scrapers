use mongodb::{Client, Database, Collection};
use mongodb::options::{ClientOptions, ServerApi, ServerApiVersion};
use anyhow::Result;
use crate::models::PriceAlert;

#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub database: Database,
}

impl MongoDb {
    pub async fn new(uri: &str, db_name: &str) -> Result<Self> {
        let mut client_options = ClientOptions::parse(uri).await?;
        
        // Set the server_api field
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        
        let client = Client::with_options(client_options)?;
        let database = client.database(db_name);
        
        // Ping the database to verify connection
        client
            .database("admin")
            .run_command(mongodb::bson::doc! {"ping": 1}, None)
            .await?;
        
        tracing::info!("Successfully connected to MongoDB");
        
        Ok(MongoDb { client, database })
    }
    
    pub fn alerts_collection(&self) -> Collection<PriceAlert> {
        self.database.collection("alerts")
    }
}
