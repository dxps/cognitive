use super::EntityDef;

#[derive(Debug)]
pub struct EntityLinkDef {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub arity: String,
    pub source_entity_def: EntityDef,
    pub target_entity_def: EntityDef,
}
