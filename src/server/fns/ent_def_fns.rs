use crate::domain::model::{EntityDef, Id};
#[cfg(feature = "server")]
use crate::server::Session;

use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

/// List the entities definitions.
#[server(endpoint = "admin/list_ent_defs", input = GetUrl)]
pub async fn list_entities_defs() -> Result<Vec<EntityDef>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.4.list().await;
    result.map_err(|e| e.into())
}

/// Create an entity definition.
#[server(endpoint = "admin/create_ent_defs")]
pub async fn create_entity_def(item: EntityDef) -> Result<Id, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.4.add(item).await;
    result.map_err(|e| e.into())
}
