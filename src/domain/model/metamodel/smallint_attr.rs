// use super::AttributeDef;
use super::{Item, ItemType};
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type boolean.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmallintAttribute {
    /// Its id.
    pub id: String,

    /// Its definition id.
    // TODO: If needed.
    // pub def: AttributeDef,

    /// Its owner type.
    // TODO: If needed.
    // pub owner_type: ItemType,

    /// Its value.
    pub value: i8,
}

impl SmallintAttribute {
    pub fn new(id: String, value: i8) -> Self {
        Self { id, value }
    }
}

impl Item for SmallintAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::SmallintAttribute
    }
}
