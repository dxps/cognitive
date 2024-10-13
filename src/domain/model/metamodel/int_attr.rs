use crate::domain::model::Id;

// use super::AttributeDef;
use super::{AttributeDef, Item, ItemType};
use serde::{Deserialize, Serialize};

/// An instance of an attribute of type integer.\
/// Its value ranges from -2147483648 to +2147483647,\
/// stored in PostgreSQL in an `int4` data type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IntegerAttribute {
    /// Its name.
    pub name: String,

    /// Its value.
    pub value: i32,

    /// Its definition id.
    pub def_id: Id,
}

impl IntegerAttribute {
    pub fn new(name: String, value: i32, def_id: Id) -> Self {
        Self { name, value, def_id }
    }
}

impl Item for IntegerAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::SmallintAttribute
    }
}

impl From<&AttributeDef> for IntegerAttribute {
    fn from(def: &AttributeDef) -> Self {
        Self::new(def.name.clone(), 0, def.id.clone())
    }
}
