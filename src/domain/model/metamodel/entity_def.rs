use crate::domain::model::Id;

use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
/// An entity definition.
pub struct EntityDef {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Vec<AttributeDef>,
    pub listing_attr_def_id: Id,
}

impl EntityDef {
    pub fn new(id: String, name: String, description: Option<String>, listing_attr_def_id: Id) -> Self {
        Self {
            id,
            name,
            description,
            attributes: vec![],
            listing_attr_def_id,
        }
    }

    pub fn new_with_attr_def_ids(
        id: String,
        name: String,
        description: Option<String>,
        attributes: Vec<Id>,
        listing_attr_id: Id,
    ) -> Self {
        Self {
            id,
            name,
            description,
            listing_attr_def_id: listing_attr_id,
            attributes: attributes.iter().map(|id| AttributeDef::new_with_id(id.clone())).collect(),
        }
    }
}

impl Item for EntityDef {
    fn item_type(&self) -> ItemType {
        ItemType::EntityDef
    }
}
