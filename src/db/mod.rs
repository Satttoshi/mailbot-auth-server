use mongodb::{Collection, Client, options::ClientOptions};
use std::env;
use dotenv::dotenv;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

pub async fn create_mongo_client() -> Result<Client, mongodb::error::Error> {
    dotenv().ok(); // Load .env file if present
    let db_uri = env::var("MONGO_URI").expect("You must set the MONGO_URI environment variable");
    let client_options = ClientOptions::parse(&db_uri).await?;
    let client = Client::with_options(client_options)?;
    Ok(client)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn get_user_collection(client: &Client) -> Collection<User> {
    let db_name = env::var("MONGO_DB_NAME").expect("You must set the MONGO_DB_NAME environment variable");
    let db = client.database(&db_name);
    db.collection::<User>("users")
}

pub async fn create_user(client: &Client, username: &str, email: &str, password: &str) -> mongodb::error::Result<()> {
    let user_collection = get_user_collection(client);
    let new_user = User {
        username: username.to_string(),
        email: email.to_string(),
        password: password.to_string(),
    };
    user_collection.insert_one(new_user, None).await?;
    Ok(())
}

pub async fn find_user_by_username(client: &Client, username: &str) -> mongodb::error::Result<Option<User>> {
    let user_collection = get_user_collection(client);
    let filter = doc! { "username": username };
    let user = user_collection.find_one(filter, None).await?;
    Ok(user)
}
