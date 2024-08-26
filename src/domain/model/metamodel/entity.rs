use super::{BooleanAttribute, EntityDef, TextAttribute};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    /// Its identifier.
    pub id: String,

    /// Its kind (that is its definition name).
    pub kind: String,

    /// Its definition.
    pub def: EntityDef,

    pub text_attributes: Vec<TextAttribute>,
    pub boolean_attributes: Vec<BooleanAttribute>,
}
