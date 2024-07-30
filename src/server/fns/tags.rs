use crate::domain::model::Tag;
use dioxus_fullstack::prelude::*;

#[cfg(feature = "server")]
use crate::server::Session;

#[server(GetTags)]
pub async fn get_tags() -> Result<Vec<Tag>, ServerFnError> {
    //
    let session: Session = extract().await?;
    let tags = session.2.list().await;
    Ok(tags)
}
