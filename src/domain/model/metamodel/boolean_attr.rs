use super::{AttributeDef, Item, ItemType};
use crate::domain::model::Id;
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type boolean.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanAttribute {
    /// Its name.
    pub name: String,

    /// Its value.
    pub value: bool,

    /// Its definition id.
    pub def_id: Id,
}

impl Item for BooleanAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::BooleanAttribute
    }
}

impl From<&AttributeDef> for BooleanAttribute {
    fn from(def: &AttributeDef) -> Self {
        Self {
            name: def.name.clone(),
            value: false,
            def_id: def.id.clone(),
        }
    }
}
