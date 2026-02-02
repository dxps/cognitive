use crate::domain::model::{AttributeTemplate, Id};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A template for an `Object`.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize, ToSchema)]
pub struct ObjectTemplate {
    pub id: Id,
    pub name: String,
    pub description: Option<String>,
    pub attr_tmpls: Vec<AttributeTemplate>,
    pub listing_attr_tmpl_id: Id,
}
