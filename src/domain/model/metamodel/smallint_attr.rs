use crate::domain::model::Id;

// use super::AttributeDef;
use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type boolean.
/// /// Its value ranges from -128 to 127.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SmallintAttribute {
    /// Its id.
    pub id: String,

    /// Its definition id.
    pub def_id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: i8,
}

impl SmallintAttribute {
    pub fn new(id: String, name: String, value: i8) -> Self {
        Self {
            id,
            def_id: Id::default(),
            name,
            value,
        }
    }
    pub fn new_with_def(id: String, def_id: Id, name: String, value: i8) -> Self {
        Self { id, def_id, name, value }
    }
}

impl Item for SmallintAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::SmallintAttribute
    }
}

impl From<&AttributeDef> for SmallintAttribute {
    fn from(def: &AttributeDef) -> Self {
        Self::new_with_def(Id::default(), def.id.clone(), def.name.clone(), 0)
    }
}
