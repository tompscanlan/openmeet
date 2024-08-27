use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub date: String,
    pub location: String,
}
