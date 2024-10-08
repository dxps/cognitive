// use super:{AttributeDef
use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextAttribute {
    /// Its id.
    pub id: String,

    /// Its name (inherited from its definition).
    pub name: String,

    /// Its definition id.
    // pub def: AttributeDef,

    /// Its owner type.
    // pub owner_type: ItemType,

    /// Its value.
    pub value: String,
}

impl TextAttribute {
    pub fn new(id: String, name: String, value: String) -> Self {
        Self { id, name, value }
    }
}

impl Item for TextAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::TextAttribute
    }
}

impl From<AttributeDef> for TextAttribute {
    fn from(attr_def: AttributeDef) -> Self {
        Self::new(attr_def.id, attr_def.name, attr_def.default_value)
    }
}
