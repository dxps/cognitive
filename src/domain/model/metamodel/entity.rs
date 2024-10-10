use super::{BooleanAttribute, EntityDef, IntegerAttribute, SmallintAttribute, TextAttribute};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    pub id: String,

    /// Its kind, that is its definition name.
    pub kind: String,

    /// Its definition.
    pub def: Option<EntityDef>,

    #[serde(default)]
    pub text_attributes: Vec<TextAttribute>,

    #[serde(default)]
    pub smallint_attributes: Vec<SmallintAttribute>,

    #[serde(default)]
    pub int_attributes: Vec<IntegerAttribute>,

    #[serde(default)]
    pub boolean_attributes: Vec<BooleanAttribute>,
}

impl Entity {
    pub fn new_from(
        kind: String,
        text_attrs: Vec<TextAttribute>,
        smallint_attrs: Vec<SmallintAttribute>,
        int_attrs: Vec<IntegerAttribute>,
        boolean_attrs: Vec<BooleanAttribute>,
    ) -> Self {
        Self {
            id: "".into(),
            kind,
            def: None,
            text_attributes: text_attrs,
            smallint_attributes: smallint_attrs,
            int_attributes: int_attrs,
            boolean_attributes: boolean_attrs,
        }
    }
}
