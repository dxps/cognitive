use super::{AttributeDef, Item, ItemType};

pub struct TextAttribute {
    /// Its id.
    pub id: String,

    /// Its definition id.
    pub def: AttributeDef,

    /// Its owner type.
    pub owner_type: ItemType,

    /// Its value.
    pub value: String,
}

impl Item for TextAttribute {
    fn item_type(&self) -> ItemType {
        ItemType::TextAttribute
    }
}
