use mongodb::{Client as MongoClient, Collection, bson::Document};
use async_trait::async_trait;
use std::error::Error;
use crate::store::storage::Storage;

pub struct MongoStorage {
    client: MongoClient,
    db: String,
}

// TODO: implement connection error handler
impl MongoStorage {
    pub async fn new(uri: &str, db: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = MongoClient::with_uri_str(uri).await?;
        println!("Connecting to the db!");
        Ok(Self {
            client,
            db: db.to_string(),
        })
    }

    fn get_collection(&self, collection_name: &str) -> Collection<Document> {
        self.client.database(&self.db).collection(collection_name)
    }
}

#[async_trait]
impl Storage for MongoStorage {
    async fn store(&self, collection_name: &str, message: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        let json_value: Document = serde_json::from_str(message)?;
        let collection = self.get_collection(collection_name);
        // println!("Inserting into the database: {}", message);
        collection.insert_one(json_value).await?;
        Ok(())
    }
}
