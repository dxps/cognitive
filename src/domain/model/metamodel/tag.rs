use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

impl Tag {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self { id, name, description }
    }
}
