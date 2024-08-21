use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
/// An entity definition.
pub struct EntityDef {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Vec<AttributeDef>,
}

impl EntityDef {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            name,
            description,
            attributes: vec![],
        }
    }
}

impl Item for EntityDef {
    fn item_type(&self) -> ItemType {
        ItemType::EntityDef
    }
}
