use crate::domain::model::{ContactPoint, Protocol, Tag};
use dioxus::prelude::*;

#[cfg(feature = "server")]
use super::Session;

#[server(PostServerData)]
pub async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    //
    tracing::info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub async fn get_server_data() -> Result<String, ServerFnError> {
    //
    Ok("Hello from the server!".to_string())
}

#[server(TestGetContactPoint)]
pub async fn test_get_contact_point() -> Result<ContactPoint, ServerFnError> {
    //
    let cp = ContactPoint {
        name: "HTTP 8080".into(),
        protocol: Protocol {
            name: "HTTP".to_string(),
            description: "HTTP".to_string(),
            required_props: vec!["URL".into()],
            optional_props: vec![],
        },
    };
    Ok(cp)
}

#[server(TestListTags)]
pub async fn test_list_tags() -> Result<Vec<Tag>, ServerFnError> {
    //
    let session: Session = extract().await?;
    let tags = session.2.list().await;
    Ok(tags)
}
