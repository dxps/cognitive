use crate::domain::model::{AttributeValueType, Id};
use serde::{Deserialize, Serialize};

/// The template for an attribute.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct AttributeTemplate {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
}
