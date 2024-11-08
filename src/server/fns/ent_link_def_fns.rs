use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

use crate::domain::model::{EntityLinkDef, Id};

#[cfg(feature = "server")]
use crate::server::Session;

/// List the entity link definitions.
#[server(endpoint = "admin/list_ent_link_defs", input = GetUrl)]
pub async fn list_entity_link_defs() -> Result<Vec<EntityLinkDef>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.6.list().await;
    result.map_err(|e| e.into())
}

/// Create an entity link definition.
#[server(endpoint = "admin/create_ent_link_def")]
pub async fn create_entity_link_def(item: EntityLinkDef) -> Result<Id, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.6.add(item).await;
    result.map_err(|e| e.into())
}
