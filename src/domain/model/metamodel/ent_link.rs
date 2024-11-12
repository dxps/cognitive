use crate::domain::model::Id;

use super::{BooleanAttribute, IntegerAttribute, SmallintAttribute, TextAttribute};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EntityLink {
    pub id: Id,

    pub source_entity_id: Id,
    pub target_entity_id: Id,

    /// Its kind, that is its definition name.
    pub kind: String,

    /// Its definition id.
    pub def_id: Id,

    // The instance attributes.
    #[serde(default)]
    pub text_attributes: Vec<TextAttribute>,

    #[serde(default)]
    pub smallint_attributes: Vec<SmallintAttribute>,

    #[serde(default)]
    pub int_attributes: Vec<IntegerAttribute>,

    #[serde(default)]
    pub boolean_attributes: Vec<BooleanAttribute>,
}

impl EntityLink {
    pub fn new(
        id: Id,
        source_entity_id: Id,
        target_entity_id: Id,
        kind: String,
        def_id: Id,
        text_attributes: Vec<TextAttribute>,
        smallint_attributes: Vec<SmallintAttribute>,
        int_attributes: Vec<IntegerAttribute>,
        boolean_attributes: Vec<BooleanAttribute>,
    ) -> Self {
        Self {
            id,
            source_entity_id,
            target_entity_id,
            kind,
            def_id,
            text_attributes,
            smallint_attributes,
            int_attributes,
            boolean_attributes,
        }
    }
}
