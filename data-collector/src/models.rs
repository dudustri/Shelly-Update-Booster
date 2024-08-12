pub mod models;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: String,
}