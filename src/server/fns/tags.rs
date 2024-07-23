use crate::domain::model::Tag;
use dioxus_fullstack::prelude::*;

#[cfg(feature = "server")]
use crate::server::Session;

#[server(TestListTags)]
pub async fn test_list_tags() -> Result<Vec<Tag>, ServerFnError> {
    //
    let session: Session = extract().await?;
    let tags = session.2.list().await;
    Ok(tags)
}
