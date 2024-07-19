use super::Tag;

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

#[derive(Debug)]
pub enum AttributeValueType {
    Text,
    Integer,
    IntegerPositiveOnly,
    Boolean,
    Date,
    DateTime,
}
