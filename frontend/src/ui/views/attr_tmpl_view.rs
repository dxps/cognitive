use crate::{
    model::Action,
    ui::{
        Route, STATE,
        components::{AttributeTemplateForm, Card},
    },
};
use dioxus::prelude::*;
use shlib::{
    AppResult,
    domain::model::{AttributeTemplate, AttributeValueType, Id, UserAccount},
};

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

    let mut name = use_signal(|| "".to_string());
    let mut description = use_signal(|| "".to_string());
    let mut value_type = use_signal(|| "text".to_string());
    let mut default_value = use_signal(|| "".to_string());
    let mut is_required = use_signal(|| false);

    let create_btn_disabled = use_memo(move || name().is_empty());
    let err: Signal<Option<String>> = use_signal(|| None);

    let iid = id.clone();
    use_future(move || {
        let iid = id.clone();
        async move {
            if let Ok(Some(item)) = get(iid.clone()).await {
                name.set(item.name.clone());
                description.set(item.description.clone().unwrap_or_default());
                value_type.set(item.value_type.to_string());
                default_value.set(item.default_value.clone());
                is_required.set(item.is_required);
            }
        }
    });

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
                        action: Action::View,
                    }
                    div { class: "grid justify-items-end mt-8",
                        button {
                            disabled: create_btn_disabled(),
                            onclick: move |_| {
                                let id = iid.clone();
                                handle_update(
                                    id,
                                    name(),
                                    description(),
                                    value_type(),
                                    default_value(),
                                    is_required(),
                                    err,
                                )
                            },
                            "Edit"
                        }
                    }
                }
            },
        }
    }
}

async fn get(id: Id) -> AppResult<Option<AttributeTemplate>> {
    //
    if let Some(item) = STATE.read().attr_tmpls_cache.get(&id).cloned() {
        Ok(Some(item))
    } else {
        match reqwest::Client::new()
            .get(format!("http://localhost:9011/data/templates/attributes/{}", id))
            .send()
            .await
        {
            Ok(rsp) => {
                if rsp.status() == reqwest::StatusCode::OK {
                    if let Ok(attr_tmpl) = rsp.json::<AttributeTemplate>().await {
                        return Ok(Some(attr_tmpl));
                    }
                }
                Ok(None)
            }
            Err(e) => AppResult::Err(e.to_string().into()),
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
