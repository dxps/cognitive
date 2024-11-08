use super::{Entity, EntityLinkDef};

pub struct EntityLink {
    pub source_entity: Entity,
    pub target_entity: Entity,
    pub def: EntityLinkDef,
}
