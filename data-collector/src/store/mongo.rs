use mongodb::{Client as MongoClient, Collection, bson::Document};
use async_trait::async_trait;
use crate::store::storage::Storage;

pub struct MongoStorage {
    collection: Collection<Document>,
}

impl MongoStorage {
    pub async fn new(uri: &str, db: &str, collection: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let client = MongoClient::with_uri_str(uri).await?;
        println!("Connecting to the db!");
        let collection = client.database(db).collection(collection);
        Ok(Self { collection })
    }
}

//TODO: figure out if it should be stored one document per energy meter or a collection per energy meter!

#[async_trait]
impl Storage for MongoStorage {
    async fn store(&self, message: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let json_value: Document = serde_json::from_str(message)?;
        println!("Inserting into the database: {}", message);
        self.collection.insert_one(json_value).await?;
        Ok(())
    }
}
