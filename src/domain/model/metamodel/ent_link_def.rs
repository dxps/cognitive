use serde::{Deserialize, Serialize};

use crate::domain::model::Id;

use super::AttributeDef;

/// The definition of an entity link.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EntityLinkDef {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub cardinality: Cardinality,
    pub source_entity_def_id: Id,
    pub target_entity_def_id: Id,
    pub attributes: Vec<AttributeDef>,
}

impl EntityLinkDef {
    pub fn new(
        id: Id,
        name: String,
        description: Option<String>,
        cardinality: Cardinality,
        source_entity_def_id: Id,
        target_entity_def_id: Id,
        attributes: Vec<AttributeDef>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            cardinality,
            source_entity_def_id,
            target_entity_def_id,
            attributes,
        }
    }
}

/// The cardinality of an entity link definition.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Cardinality {
    OneToOne,
    OneToMany,
    ManyToMany,
}

impl Cardinality {
    //
    pub fn to_string(&self) -> String {
        match self {
            Cardinality::OneToOne => "1:1".to_string(),
            Cardinality::OneToMany => "1:M".to_string(),
            Cardinality::ManyToMany => "M:M".to_string(),
        }
    }
}

impl From<&str> for Cardinality {
    fn from(value: &str) -> Self {
        match value {
            "1:1" => Cardinality::OneToOne,
            "1:M" => Cardinality::OneToMany,
            "M:M" => Cardinality::ManyToMany,
            _ => Cardinality::OneToOne,
        }
    }
}
