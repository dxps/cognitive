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
pub fn AttributeTemplateView(id: Id) -> Element {
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
    let err: Signal<Option<String>> = use_signal(|| None);

    rsx! {
        Card {
            header: rsx! {
                h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)", "Attribute Template" }
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
                                handle_update(
                                    id.clone(),
                                    name(),
                                    description(),
                                    value_type(),
                                    default_value(),
                                    is_required(),
                                    err,
                                )
                            },
                            "Create"
                        }
                    }
                }
            },
        }
    }
}

async fn handle_update(
    id: Id,
    name: String,
    description: String,
    value_type: String,
    default_value: String,
    is_required: bool,
    mut err: Signal<Option<String>>,
) {
    //
    let item = AttributeTemplate {
        id,
        name,
        description: if description.is_empty() { None } else { Some(description) },
        value_type: AttributeValueType::from(value_type),
        default_value,
        is_required,
    };
    match reqwest::Client::new()
        .put(format!("http://localhost:9011/data/templates/attributes/{}", item.id))
        .json(&item)
        .send()
        .await
    {
        Ok(rsp) => {
            if rsp.status() == reqwest::StatusCode::OK {
                if let Ok(attr_tmpl) = rsp.json::<AttributeTemplate>().await {
                    STATE.write().attr_tmpls_cache.insert(attr_tmpl.id.clone(), attr_tmpl);
                    use_navigator().push(Route::AttributeTemplatesListView {});
                }
            } else {
                let msg = format!("Failed to add attribute template. Got HTTP status: {}", rsp.status());
                error!(msg);
                err.set(Some(msg));
            }
        }
        Err(e) => {
            let msg = format!("Failed to add attribute template. Reason: {}", e);
            error!(msg);
            err.set(Some(msg));
        }
    }
}
