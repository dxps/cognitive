use serde::{Deserialize, Serialize};

pub trait Item {
    fn item_type(&self) -> ItemType;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    Tag,
    AttributeDef,
    EntityDef,
    EntityLinkDef,
    Entity,
    EntityLink,
    TextAttribute,
    BooleanAttribute,
}

impl ItemType {
    pub fn value(&self) -> String {
        match self {
            ItemType::Tag => "tag".to_string(),
            ItemType::AttributeDef => "atd".to_string(),
            ItemType::EntityDef => "end".to_string(),
            ItemType::EntityLinkDef => "eld".to_string(),
            ItemType::Entity => "eni".to_string(),
            ItemType::EntityLink => "enl".to_string(),
            ItemType::TextAttribute => "tat".to_string(),
            ItemType::BooleanAttribute => "tat".to_string(),
        }
    }
}
