use crate::domain::model::Id;

use super::{BooleanAttribute, IntegerAttribute, SmallintAttribute, TextAttribute};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub id: Id,

    /// Its kind, that is its definition name.
    pub kind: String,

    /// Its definition.
    pub def_id: Id,

    #[serde(default)]
    pub text_attributes: Vec<TextAttribute>,

    #[serde(default)]
    pub smallint_attributes: Vec<SmallintAttribute>,

    #[serde(default)]
    pub int_attributes: Vec<IntegerAttribute>,

    #[serde(default)]
    pub boolean_attributes: Vec<BooleanAttribute>,

    pub listing_attr_def_id: Id,
    pub listing_attr_name: String,
    pub listing_attr_value: String,
}

impl Entity {
    pub fn new(
        kind: String,
        def_id: Id,
        text_attributes: Vec<TextAttribute>,
        smallint_attributes: Vec<SmallintAttribute>,
        int_attributes: Vec<IntegerAttribute>,
        boolean_attributes: Vec<BooleanAttribute>,
        listing_attr_def_id: Id,
        listing_attr_name: String,
        listing_attr_value: String,
    ) -> Self {
        Self {
            id: Id::default(),
            kind,
            def_id: def_id,
            text_attributes,
            smallint_attributes,
            int_attributes,
            boolean_attributes,
            listing_attr_def_id,
            listing_attr_name,
            listing_attr_value,
        }
    }

    pub fn new_with_id_attrs(
        id: Id,
        kind: String,
        def_id: Id,
        text_attributes: Vec<TextAttribute>,
        smallint_attributes: Vec<SmallintAttribute>,
        int_attributes: Vec<IntegerAttribute>,
        boolean_attributes: Vec<BooleanAttribute>,
        listing_attr_def_id: Id,
    ) -> Self {
        Self {
            id,
            kind,
            def_id,
            text_attributes,
            smallint_attributes,
            int_attributes,
            boolean_attributes,
            listing_attr_def_id,
            // The following values are not relevant since this function is used only in the entity update use case.
            listing_attr_name: String::default(),
            listing_attr_value: String::default(),
        }
    }
}
