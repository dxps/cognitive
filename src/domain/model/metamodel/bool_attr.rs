use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type boolean.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanAttribute {
    /// Its id.
    pub id: String,

    /// Its definition id.
    pub def: AttributeDef,

    /// Its owner type.
    pub owner_type: ItemType,

    /// Its value.
    pub value: bool,
}

impl Item for BooleanAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::BooleanAttribute
    }
}
