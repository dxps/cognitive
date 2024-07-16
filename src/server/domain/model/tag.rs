use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub description: String,
}
