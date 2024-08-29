use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextAttribute {
    /// Its id.
    pub id: String,

    /// Its definition id.
    // pub def: AttributeDef,

    /// Its owner type.
    // pub owner_type: ItemType,

    /// Its value.
    pub value: String,
}

impl TextAttribute {
    pub fn new(id: String, value: String) -> Self {
        Self { id, value }
    }
}

impl Item for TextAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::TextAttribute
    }
}
