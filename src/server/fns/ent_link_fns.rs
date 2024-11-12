use dioxus_fullstack::prelude::*;
use server_fn::codec::{GetUrl, PostUrl};

use crate::domain::model::{EntityLink, EntityLinkDef, Id};

#[cfg(feature = "server")]
use crate::server::Session;

/// List the entity links.
#[server(endpoint = "admin/list_ent_links", input = GetUrl)]
pub async fn list_entity_links() -> Result<Vec<EntityLink>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.7.list().await;
    result.map_err(|e| e.into())
}

// /// Create an entity link.
// #[server(endpoint = "admin/create_ent_link", input = PostUrl)]
// pub async fn create_entity_link_def(item: EntityLink) -> Result<Id, ServerFnError> {
//     let session: Session = extract().await?;
//     let result = session.6.add(item).await;
//     result.map_err(|e| e.into())
// }

// /// Get an entity link.
// #[server(endpoint = "admin/get_ent_link", input = GetUrl)]
// pub async fn get_entity_link(id: Id) -> Result<Option<EntityLink>, ServerFnError> {
//     let session: Session = extract().await?;
//     let ent_link_def = session.6.get(&id).await?;
//     Ok(ent_link_def)
// }

// /// Update an entity link.
// #[server(endpoint = "admin/update_ent_link")]
// pub async fn update_entity_link(ent_link_def: EntityLink) -> Result<(), ServerFnError> {
//     let session: Session = extract().await?;
//     let result = session.6.update(&ent_link_def).await;
//     result.map_err(|e| e.into())
// }

// /// Remove an entity link definition.
// #[server(endpoint = "admin/remove_ent_link", input = PostUrl)]
// pub async fn remove_entity_link(id: Id) -> Result<(), ServerFnError> {
//     let session: Session = extract().await?;
//     let result = session.6.remove(&id).await;
//     result.map_err(|e| e.into())
// }
