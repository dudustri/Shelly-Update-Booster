pub mod storage;
pub mod mongo;

use crate::store::mongo::MongoStorage;
use crate::store::storage::Storage;

pub async fn init_storage(uri: &str, db: &str) -> Result<Box<dyn Storage>, Box<dyn std::error::Error + Send + Sync>> {
    let mongo_storage = MongoStorage::new(&uri, &db).await?;
    Ok(Box::new(mongo_storage))
}
