use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

use crate::domain::model::{AttributeDef, Id};
#[cfg(feature = "server")]
use crate::server::Session;

/// List the attribute definitions.
#[server(endpoint = "admin/list_attr_defs", input = GetUrl)]
pub async fn list_attribute_defs() -> Result<Vec<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_defs = session.3.list().await;
    Ok(attr_defs)
}

/// Get an attribute definitions.
#[server(endpoint = "admin/get_attr_def", input = GetUrl)]
pub async fn get_attribute_def(id: String) -> Result<Option<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_def = session.3.get(&id).await;
    Ok(attr_def)
}

/// Create an attribute definition.
#[server(endpoint = "admin/create_attr_def")]
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

/// Create an attribute definition.
#[server(endpoint = "admin/update_attr_def")]
pub async fn update_attribute_def(item: AttributeDef) -> Result<(), ServerFnError> {
    //
    log::debug!("Updating attribute def: {:?}", item);
    let session: Session = extract().await?;
    session.3.update(item).await.map(|_| Ok(()))?
}
