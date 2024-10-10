use crate::domain::model::Id;

// use super::AttributeDef;
use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type integer.\
/// Its value ranges from -2147483648 to +2147483647,\
/// stored in PostgreSQL in an `int4` data type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegerAttribute {
    /// Its id.
    pub id: String,

    /// Its definition id.
    pub def_id: Id,

    /// Its name.
    pub name: String,

    /// Its value.
    pub value: i32,
}

impl IntegerAttribute {
    pub fn new(id: Id, name: String, value: i32) -> Self {
        Self {
            id,
            def_id: Id::default(),
            name,
            value,
        }
    }
    pub fn new_with_def(def_id: Id, name: String, value: i32) -> Self {
        Self {
            id: Id::default(),
            def_id,
            name,
            value,
        }
    }
}

impl Item for IntegerAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::SmallintAttribute
    }
}

impl From<&AttributeDef> for IntegerAttribute {
    fn from(def: &AttributeDef) -> Self {
        Self::new_with_def(def.id.clone(), def.name.clone(), 0)
    }
}
