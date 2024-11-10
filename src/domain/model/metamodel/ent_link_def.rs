use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{domain::model::Id, ui::pages::Name};

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
    pub attributes: Option<Vec<AttributeDef>>,
}

impl EntityLinkDef {
    pub fn new(
        id: Id,
        name: String,
        description: Option<String>,
        cardinality: Cardinality,
        source_entity_def_id: Id,
        target_entity_def_id: Id,
        attributes: Option<Vec<AttributeDef>>,
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

    pub fn from(
        name: String,
        description: Option<String>,
        cardinality: Cardinality,
        source_entity_def_id: Id,
        target_entity_def_id: Id,
        attributes: Option<Vec<AttributeDef>>,
    ) -> Self {
        Self::new(
            Id::new(),
            name,
            description,
            cardinality,
            source_entity_def_id,
            target_entity_def_id,
            attributes,
        )
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

    /// Get the (enum) variants.
    pub fn get_select_variants() -> HashMap<Id, Name> {
        HashMap::from([
            (Id::from(Self::OneToOne.to_string()), Self::OneToOne.to_string()),
            (Id::from(Self::OneToMany.to_string()), Self::OneToMany.to_string()),
            (Id::from(Self::ManyToMany.to_string()), Self::ManyToMany.to_string()),
        ])
    }
}

impl Default for Cardinality {
    fn default() -> Self {
        Self::OneToOne
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
