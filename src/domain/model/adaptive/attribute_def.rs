use crate::domain::model::Tag;

use super::{Item, ItemType};

#[derive(Debug)]
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

#[derive(Debug)]
/// The type of a value of an attribute.
pub enum AttributeValueType {
    /// This is mapped to PostgreSQL's `varchar(10485760)` (not `text`) type.
    Text,

    /// This is mapped to PostgreSQL's `smallint` type. The range is `[-32768, 32767]`.
    Integer16bit,

    /// This is mapped to PostgreSQL's `integer` type. The range is `[-2147483648, 2147483647]`.
    Integer32bit,

    /// This is mapped to PostgreSQL's `bigint` type. The range is `[-9223372036854775808, 9223372036854775807]`.
    Integer64bit,

    /// This is mapped to PostgreSQL's `real` type. The range is `[-3.402823466E+38, 3.402823466E+38]`.
    Decimal32bit,

    /// This is mapped to PostgreSQL's `boolean` type. The range is `[false, true]`.
    Boolean,

    /// This is mapped to PostgreSQL's `date` type.
    Date,

    /// This is mapped to PostgreSQL's `timestamp` (without time zone) type.
    DateTime,
}
