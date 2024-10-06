use crate::domain::model::{Entity, Id};

#[cfg(feature = "server")]
use crate::server::Session;

use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

/// List the entities instances.
#[server(endpoint = "admin/list_ents", input = GetUrl)]
pub async fn list_entities() -> Result<Vec<Entity>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.5.list().await;
    result.map_err(|e| e.into())
}

/// Create an entity instance.
#[server(endpoint = "admin/create_ent")]
pub async fn create_entity(item: Entity) -> Result<Id, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.5.add(item).await;
    result.map_err(|e| e.into())
}

/// Get an entity instance.
#[server(endpoint = "admin/get_ent", input = GetUrl)]
pub async fn get_entity(id: Id) -> Result<Option<Entity>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.5.get(&id).await;
    result.map_err(|e| e.into())
}

/// Update an entity instance.
#[server(endpoint = "admin/update_ent")]
pub async fn update_entity(ent_def: Entity) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.5.update(ent_def).await;
    result.map_err(|e| e.into())
}

/// Remove an entity instance.
#[server(endpoint = "admin/remove_ent")]
pub async fn remove_entity(id: Id) -> Result<(), ServerFnError> {
    let session: Session = extract().await?;
    let result = session.5.remove(&id).await;
    result.map_err(|e| e.into())
}
