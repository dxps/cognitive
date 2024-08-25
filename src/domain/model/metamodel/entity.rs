use super::EntityDef;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Entity {
    /// Its identifier.
    pub id: String,

    /// Its kind (that is its definition name).
    pub kind: String,

    /// Its definition.
    pub def: EntityDef,
}
