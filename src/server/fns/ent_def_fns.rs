use crate::domain::model::EntityDef;
#[cfg(feature = "server")]
use crate::server::Session;

use dioxus_fullstack::prelude::*;
use server_fn::codec::GetUrl;

/// List the entities definitions.
#[server(endpoint = "admin/list_ent_defs", input = GetUrl)]
pub async fn list_entities_defs() -> Result<Vec<EntityDef>, ServerFnError> {
    let session: Session = extract().await?;
    let ent_defs_result = session.4.list().await;
    ent_defs_result.map_err(|e| e.into())
}
