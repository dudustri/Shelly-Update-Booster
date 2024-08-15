#[derive(Clone)]
pub struct StoreTaskMessage {
    pub collection: String,
    pub payload: String,
}