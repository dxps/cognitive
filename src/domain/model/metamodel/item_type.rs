pub trait Item {
    fn item_type(&self) -> ItemType;
}

pub enum ItemType {
    Tag,
    AttributeDef,
    EntityDef,
    EntityLinkDef,
    Entity,
    EntityLink,
    TextAttribute,
}

impl ItemType {
    pub fn value(&self) -> String {
        match self {
            ItemType::Tag => "tag".to_string(),
            ItemType::AttributeDef => "atd".to_string(),
            ItemType::EntityDef => "end".to_string(),
            ItemType::EntityLinkDef => "eld".to_string(),
            ItemType::Entity => "eni".to_string(),
            ItemType::EntityLink => "enl".to_string(),
            ItemType::TextAttribute => "tat".to_string(),
        }
    }
}

// --------------------------------------------------------------------------------
// An alternative (using a struct and associated constants), to be evaluated later.
// --------------------------------------------------------------------------------

// #[non_exhaustive]
// pub struct ItemType;

// impl ItemType {
//     /// A tag is used for tagging other items, such as attribute definitions or entity definitions.
//     #[allow(non_upper_case_globals)]
//     pub const Tag: i8 = 0;

//     /// An attribute definition.
//     #[allow(non_upper_case_globals)]
//     pub const AttributeDef: i8 = 1;

//     /// An entity definition.
//     #[allow(non_upper_case_globals)]
//     pub const EntityDef: i8 = 2;

//     /// A definition of a link between two entities.
//     #[allow(non_upper_case_globals)]
//     pub const EntityLinkDef: i8 = 3;

//     /// An instance of an entity.
//     #[allow(non_upper_case_globals)]
//     pub const Entity: i8 = 4;

//     /// An instance of a link between two entities.
//     #[allow(non_upper_case_globals)]
//     pub const EntityLink: i8 = 5;

//     /// An attribute of type "text".
//     #[allow(non_upper_case_globals)]
//     pub const TextAttribute: i8 = 6;
// }
