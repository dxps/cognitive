use crate::domain::model::Id;
use serde::{Deserialize, Serialize};
use strum::Display;

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

/// The type of the value of an attribute.
#[derive(Debug, Default, PartialEq, Clone, Display, Serialize, Deserialize)]
pub enum AttributeValueType {
    //
    /// This is mapped to PostgreSQL's `TEXT` data type.
    #[default]
    #[strum(to_string = "text")]
    Text,

    /// This is mapped to PostgreSQL's `NUMERIC` data type.
    #[strum(to_string = "numeric")]
    Numeric,

    /// This is mapped to PostgreSQL's `DATE` data type.
    #[strum(to_string = "date")]
    Date,

    /// This is mapped to PostgreSQL's `TIMESTAMP` (without time zone) data type.
    #[strum(to_string = "timestamp")]
    DateTime,
}
