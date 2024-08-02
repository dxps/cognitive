use dioxus_fullstack::prelude::*;

use crate::domain::model::{AttributeDef, Id};
#[cfg(feature = "server")]
use crate::server::Session;

/// Get all the attribute definitions.
#[server(GetAttributeDefs)]
pub async fn get_attribute_defs() -> Result<Vec<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_defs = session.3.list().await;
    Ok(attr_defs)
}

/// Get an attribute definitions.
#[server(GetAttributeDef)]
pub async fn get_attribute_def(id: String) -> Result<Option<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_def = session.3.get(&id).await;
    Ok(attr_def)
}

/// Create an attribute definition.
#[server]
pub async fn create_attribute_def(
    name: String,
    description: String,
    value_type: String,
    default_value: String,
    is_required: bool,
    is_multivalued: bool,
    tag_id: String,
) -> Result<Id, ServerFnError> {
    //
    let session: Session = extract().await?;
    session
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
        .await
        .map(|id| Ok(id))?
}
