use dioxus_fullstack::prelude::*;

use crate::domain::model::AttributeDef;
#[cfg(feature = "server")]
use crate::server::Session;

#[server(ListAttributeDefs)]
pub async fn list_attribute_defs() -> Result<Vec<AttributeDef>, ServerFnError> {
    let session: Session = extract().await?;
    let attr_defs = session.3.list().await;
    Ok(attr_defs)
}
