use serde::{Deserialize, Serialize};
use strum::Display;

use crate::domain::model::Tag;

use super::{Item, ItemType};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
/// An attribute definition.
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

impl Item for AttributeDef {
    fn item_type(&self) -> ItemType {
        ItemType::AttributeDef
    }
}

#[derive(Debug, PartialEq, Clone, Display, Serialize, Deserialize)]
/// The type of a value of an attribute.
pub enum AttributeValueType {
    /// This is mapped to PostgreSQL's `text` type.
    Text,

    /// This is mapped to PostgreSQL's `smallint` type. The range is `[-32768, 32767]`.
    SmallInteger, // used as Rust u16

    /// This is mapped to PostgreSQL's `integer` type. The range is `[-2147483648, 2147483647]`.
    Integer, // used as Rust u32

    /// This is mapped to PostgreSQL's `bigint` type. The range is `[-9223372036854775808, 9223372036854775807]`.
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
