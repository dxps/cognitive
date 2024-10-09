use super::{BooleanAttribute, EntityDef, IntegerAttribute, TextAttribute};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub id: String,

    /// Its kind, that is its definition name.
    pub kind: String,

    /// Its definition.
    pub def: Option<EntityDef>,

    pub text_attributes: Vec<TextAttribute>,
    pub smallint_attributes: Vec<IntegerAttribute>,
    pub boolean_attributes: Vec<BooleanAttribute>,
}

impl Entity {
    pub fn new_from(kind: String, text_attrs: Vec<TextAttribute>) -> Self {
        Self {
            id: "".into(),
            kind,
            def: None,
            text_attributes: text_attrs,
            smallint_attributes: vec![],
            boolean_attributes: vec![],
        }
    }
}
