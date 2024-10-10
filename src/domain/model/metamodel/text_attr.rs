use crate::domain::model::Id;

use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextAttribute {
    /// Its name (inherited from its definition).
    pub name: String,

    /// Its value.
    pub value: String,

    /// Its definition id.
    pub def_id: Id,

    /// Its owner id.
    pub owner_id: Id,

    /// Its owner type.
    pub owner_type: ItemType,
}

impl TextAttribute {
    pub fn new(name: String, value: String, def_id: String, owner_id: Id, owner_type: ItemType) -> Self {
        Self {
            name,
            value,
            def_id,
            owner_id,
            owner_type,
        }
    }
}

impl Item for TextAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::TextAttribute
    }
}

impl From<AttributeDef> for TextAttribute {
    fn from(attr_def: AttributeDef) -> Self {
        Self::new(
            attr_def.name,
            attr_def.default_value,
            attr_def.id,
            Id::default(),
            ItemType::Unknown,
        )
    }
}
