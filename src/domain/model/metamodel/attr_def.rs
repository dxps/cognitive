use super::{Item, ItemType};
use crate::domain::model::Id;
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
/// An attribute definition.
pub struct AttributeDef {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub value_type: AttributeValueType,
    pub default_value: String,
    pub is_multivalued: bool,
    pub is_required: bool,
    pub tag_id: Option<Id>,
}

impl AttributeDef {
    pub fn new(
        id: Id,
        name: String,
        description: Option<String>,
        value_type: AttributeValueType,
        default_value: String,
        is_multivalued: bool,
        is_required: bool,
        tag_id: Option<Id>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            value_type,
            default_value,
            is_multivalued,
            is_required,
            tag_id,
        }
    }
}

impl Item for AttributeDef {
    fn item_type(&self) -> ItemType {
        ItemType::AttributeDef
    }
}

#[derive(Debug, Default, PartialEq, Clone, Display, Serialize, Deserialize)]
/// The type of a value of an attribute.
pub enum AttributeValueType {
    //
    /// This is mapped to PostgreSQL's `text` type.
    #[default]
    Text,

    /// This is mapped to PostgreSQL's `smallint` type. The range is `[-32768, 32767]`.
    #[strum(to_string = "Small Integer")]
    SmallInteger, // used as Rust u16

    /// This is mapped to PostgreSQL's `integer` type. The range is `[-2147483648, 2147483647]`.
    Integer, // used as Rust u32

    /// This is mapped to PostgreSQL's `bigint` type. The range is `[-9223372036854775808, 9223372036854775807]`.
    #[strum(to_string = "Big Integer")]
    BigInteger, // used as Rust u64

    /// This is mapped to PostgreSQL's `real` type. The range is `[-3.402823466E+38, 3.402823466E+38]`.
    Decimal, // Decimal32bit

    /// This is mapped to PostgreSQL's `boolean` type.
    Boolean,

    /// This is mapped to PostgreSQL's `date` type.
    Date,

    /// This is mapped to PostgreSQL's `timestamp` (without time zone) type.
    DateTime,
}

impl From<&str> for AttributeValueType {
    fn from(value: &str) -> Self {
        // FYI: As before, these string values represent PostgreSQL's types.
        match value {
            "text" => Self::Text,
            "smallint" => Self::SmallInteger,
            "integer" => Self::Integer,
            "bigint" => Self::BigInteger,
            "real" => Self::Decimal,
            "boolean" => Self::Boolean,
            "date" => Self::Date,
            "timestamp" => Self::DateTime,
            _ => Self::Text,
        }
    }
}

impl From<String> for AttributeValueType {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}
