use super::{AttributeDef, Item, ItemType};
use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type boolean.
/// Its value ranges from -128 to 127.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmallintAttribute {
    /// Its name.
    pub name: String,

    /// Its value.
    pub value: i8,

    /// Its definition id.
    pub def_id: Id,
}

impl SmallintAttribute {
    pub fn new(name: String, value: i8, def_id: Id) -> Self {
        Self { name, value, def_id }
    }
}

impl Item for SmallintAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::SmallintAttribute
    }
}

impl From<&AttributeDef> for SmallintAttribute {
    fn from(def: &AttributeDef) -> Self {
        Self::new(def.name.clone(), 0, def.id.clone())
    }
}
