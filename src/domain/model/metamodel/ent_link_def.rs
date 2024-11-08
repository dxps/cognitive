use serde::{Deserialize, Serialize};

use crate::domain::model::Id;

/// The definition of an `EntityLink`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct EntityLinkDef {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub cardinality: String,
    pub source_entity_def_id: Id,
    pub target_entity_def_id: Id,
}

impl EntityLinkDef {
    pub fn new(
        id: Id,
        name: String,
        description: Option<String>,
        cardinality: String,
        source_entity_def_id: Id,
        target_entity_def_id: Id,
    ) -> Self {
        Self {
            id,
            name,
            description,
            cardinality,
            source_entity_def_id,
            target_entity_def_id,
        }
    }
}
