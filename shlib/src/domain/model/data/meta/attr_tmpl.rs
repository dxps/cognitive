use crate::domain::model::{AttributeValueType, Id};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The template for an attribute.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, ToSchema)]
pub struct AttributeTemplate {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_required: bool,
}
