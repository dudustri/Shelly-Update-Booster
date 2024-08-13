pub mod models;

use serde::{Serialize, Deserialize};


// TODO: change the topic to be either a collection or a document - needs to be defined. Chose the fastest one for the queries.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub topic: String,
    pub payload: String,
}