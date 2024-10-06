use crate::domain::model::{EntityDef, Id};

#[cfg(feature = "server")]
use crate::server::Session;

use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

/// List the entities definitions names, these being entities kinds.
#[server(endpoint = "admin/list_ent_defs_id_name", input = GetUrl)]
pub async fn list_entities_defs_id_name() -> Result<Vec<(Id, String)>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.4.list_ids_names().await;
    std::thread::sleep(std::time::Duration::from_secs(3));
    result.map_err(|e| e.into())
}

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

/// Get an entity definition.
#[server(endpoint = "admin/get_ent_def", input = GetUrl)]
pub async fn get_entity_def(id: Id) -> Result<Option<EntityDef>, ServerFnError> {
    let session: Session = extract().await?;
    let ent_def = session.4.get(&id).await;
    Ok(ent_def)
}

/// Update an entity definition.
#[server(endpoint = "admin/update_ent_defs")]
pub async fn update_entity_def(ent_def: EntityDef) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.4.update(ent_def).await;
    result.map_err(|e| e.into())
}

/// Remove an entity definition.
#[server(endpoint = "admin/remove_ent_defs")]
pub async fn remove_entity_def(id: Id) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.4.remove(&id).await;
    result.map_err(|e| e.into())
}
