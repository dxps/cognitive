use dioxus_fullstack::prelude::*;

use crate::domain::model::AttributeDef;
#[cfg(feature = "server")]
use crate::server::Session;

#[server(ListAttributeDefs)]
pub async fn get_attribute_defs() -> Result<Vec<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_defs = session.3.list().await;
    Ok(attr_defs)
}

#[server(PostCreateAttributeDef)]
pub async fn create_attribute_def(
    name: String,
    description: String,
    value_type: String,
    default_value: String,
    is_required: bool,
    is_multivalued: bool,
    tag_id: String,
) -> Result<AttributeDef, ServerFnError> {
    let session: Session = extract().await?;
    let attr_def = session
        .3
        .add(
            name,
            description,
            value_type,
            default_value,
            is_required,
            is_multivalued,
            tag_id,
        )
        .await;
    Ok(attr_def)
}
