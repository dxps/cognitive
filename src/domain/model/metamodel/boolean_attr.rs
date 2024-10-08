use crate::domain::model::Id;

use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type boolean.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BooleanAttribute {
    /// Its id.
    pub id: String,

    /// Its definition id.
    pub def_id: Id,

    /// Its value.
    pub value: bool,
}

impl Item for BooleanAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::BooleanAttribute
    }
}

impl From<&AttributeDef> for BooleanAttribute {
    fn from(def: &AttributeDef) -> Self {
        Self {
            id: "".to_string(),
            def_id: def.id.clone(),
            value: false,
        }
    }
}
