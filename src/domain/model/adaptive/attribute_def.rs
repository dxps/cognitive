use crate::domain::model::Tag;

use super::{Item, ItemType};

#[derive(Debug)]
/// An attribute definition.
pub struct AttributeDef {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_multivalued: bool,
    pub is_composite: bool,
    pub is_required: bool,
    pub value_rules: Option<String>,
    pub tag: Option<Tag>,
}

impl Item for AttributeDef {
    fn item_type(&self) -> ItemType {
        ItemType::AttributeDef
    }
}

#[derive(Debug)]
/// The type of a value of an attribute.
pub enum AttributeValueType {
    Text,
    // TODO: not-yet-implemented
    Integer,
    // TODO: not-yet-implemented
    IntegerPositiveOnly,
    // TODO: not-yet-implemented
    Boolean,
    // TODO: not-yet-implemented
    Date,
    // TODO: not-yet-implemented
    DateTime,
}
