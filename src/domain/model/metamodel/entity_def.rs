use super::{AttributeDef, Item, ItemType};
use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

/// An entity definition.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct EntityDef {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub attributes: Vec<AttributeDef>,
    pub listing_attr_def_id: Id,
}

impl EntityDef {
    pub fn new(id: Id, name: String, description: Option<String>, listing_attr_def_id: Id) -> Self {
        Self {
            id,
            name,
            description,
            attributes: vec![],
            listing_attr_def_id,
        }
    }

    pub fn new_with_attr_def_ids(
        id: Id,
        name: String,
        description: Option<String>,
        attributes: Vec<Id>,
        listing_attr_def_id: Id,
    ) -> Self {
        Self {
            id,
            name,
            description,
            listing_attr_def_id,
            attributes: attributes.iter().map(|id| AttributeDef::new_with_id(id.clone())).collect(),
        }
    }
}

impl Item for EntityDef {
    fn item_type(&self) -> ItemType {
        ItemType::EntityDef
    }
}
