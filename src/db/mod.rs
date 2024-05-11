use mongodb::{Client, options::ClientOptions};
use std::env;
use dotenv::dotenv;

pub async fn create_mongo_client() -> Result<Client, mongodb::error::Error> {
    dotenv().ok(); // Load .env file if present
    let db_uri = env::var("MONGO_URI").expect("You must set the MONGO_URI environment variable");
    let client_options = ClientOptions::parse(&db_uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}
