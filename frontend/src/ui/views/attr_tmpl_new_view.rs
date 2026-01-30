use crate::{
    model::Action,
    ui::{
        Route, STATE,
        components::{AttributeTemplateForm, Card},
    },
};
use dioxus::prelude::*;
use shlib::domain::model::{AttributeTemplate, AttributeValueType, Id, UserAccount};

#[component]
pub fn AttributeTemplateNewView() -> Element {
    //
    let user_account = STATE.read().user.clone().unwrap_or_else(|| {
        use_navigator().push(Route::LoginView {});
        UserAccount::default()
    });
    if !user_account.is_admin() {
        use_navigator().push(Route::HomeView {});
    }

    let name = use_signal(|| "".to_string());
    let description = use_signal(|| "".to_string());
    let value_type = use_signal(|| "text".to_string());
    let default_value = use_signal(|| "".to_string());
    let is_required = use_signal(|| false);

    let create_btn_disabled = use_memo(move || name().is_empty());
    let mut err: Signal<Option<String>> = use_signal(|| None);
    let mut action_done = use_signal(|| false);

    rsx! {
        Card {
            header: rsx! {
                h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)",
                    "New Template for an Attribute"
                }
            },
            content: rsx! {
                div {
                    AttributeTemplateForm {
                        name,
                        description,
                        value_type,
                        default_value,
                        is_required,
                        action: Action::Create,
                    }
                    div { class: "grid justify-items-end mt-8",
                        button {
                            disabled: create_btn_disabled(),
                            onclick: move |_| {
                                handle_add(name(), description(), value_type(), default_value(), is_required())
                            },
                            if action_done() {
                                "Close"
                            }
                            "Create"
                        }
                    }
                }
            },
        }
    }
}

async fn handle_add(name: String, description: String, value_type: String, default_value: String, is_required: bool) {
    //
    let item = AttributeTemplate {
        id: Id::default(),
        name,
        description: if description.is_empty() { None } else { Some(description) },
        value_type: AttributeValueType::from(value_type),
        default_value,
        is_required,
    };
    match reqwest::Client::new()
        .post("http://localhost:9011/data/templates/attributes")
        .json(&item)
        .send()
        .await
    {
        Ok(rsp) => {
            if rsp.status() == reqwest::StatusCode::CREATED {
                if let Ok(attr_tmpl) = rsp.json::<AttributeTemplate>().await {
                    use_navigator().push(Route::AttributeTemplateView { id: attr_tmpl.id });
                }
            } else {
                error!("Failed to add attribute template. Reason: '{}'.", rsp.status());
            }
        }
        Err(e) => {
            error!("Failed to send request to add attribute template. Reason: '{}'.", e);
        }
    }
}
