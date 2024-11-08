use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

use crate::domain::model::EntityLinkDef;

#[cfg(feature = "server")]
use crate::server::Session;

/// List the entity link definitions.
#[server(endpoint = "admin/list_ent_link_defs", input = GetUrl)]
pub async fn list_entity_link_defs() -> Result<Vec<EntityLinkDef>, ServerFnError> {
    let session: Session = extract().await?;
    let result = session.6.list().await;
    result.map_err(|e| e.into())
}
