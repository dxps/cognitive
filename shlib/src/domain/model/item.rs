use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ItemType {
    ObjectTemplate,
    AttributeTemplate,
    LinkDef,
    Object,
    Attribute,
    Link,
}
