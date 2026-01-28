use crate::ui::{Route, STATE};
use dioxus::prelude::*;
use shlib::domain::model::{AttributeTemplate, UserAccount};

#[component]
pub fn AttributeTemplatesListView() -> Element {
    //
    let state = STATE.write();
    let user_account = state.user.clone().unwrap_or_else(|| {
        use_navigator().push(Route::HomeView {});
        UserAccount::default()
    });
    if !user_account.is_admin() {
        use_navigator().push(Route::HomeView {});
    }

    let mut entries = use_signal::<Vec<AttributeTemplate>>(|| state.attr_tmpls_cache.clone());

    let attr_tmpls_not_fetched = state.attr_tmpls_cache.is_empty();
    log::info!(">>> [AttributeTemplatesListView] attr_tmpls_not_fetched={}", attr_tmpls_not_fetched);

    use_effect(move || {
        if attr_tmpls_not_fetched {
            spawn(async move {
                let fetched_entries = fetch_attribute_templates().await;
                STATE.write().attr_tmpls_cache = fetched_entries.clone();
                entries.set(fetched_entries);
            });
        }
    });

    rsx! {
        div { class: "pt-[var(--nav-height)] min-h-[calc(100vh-var(--nav-height))] flex",
            div { class: "flex flex-col grow justify-center items-center py-6 drop-shadow-2xl",
                div { class: "bg-white dark:bg-(--dark-bg-d1) rounded-lg p-4 sm:min-w-[600px] sm:min-h-[500px]",
                    h1 { class: "text-xl text-center text-(--fg-item) dark:text-(--dark-fg-item)",
                        "Attribute Templates"
                    }

                    table { class: "w-full mt-8 mb-4 text-sm text-left rounded-lg overflow-hidden",
                        thead { class: "text-xs text-(--fg-item) bg-(--bg) dark:text-(--dark-fg-item) dark:bg-(--dark-bg-l1)",
                            tr {
                                td { class: "px-2 py-2", "name" }
                                td { class: "px-2 py-2", "description" }
                            }
                        }
                        tbody {
                            {
                                entries
                                    .iter()
                                    .map(|attr_tmpl| {
                                        let id = attr_tmpl.id.clone();
                                        rsx! {
                                            tr {
                                                key: "{attr_tmpl.id}",
                                                onclick: move |_| {
                                                    use_navigator()
                                                        .push(Route::AttributeTemplateView {
                                                            id: id.clone(),
                                                        });
                                                },
                                                class: "hover:bg-gray-100 font-medium text-(--fg-item) hover:text-(--fg-link) hover:bg(--bg-item-hover) dark:text-(--dark-fg-item) dark:hover:bg-(--dark-bg-item-hover) dark:hover:text-(--dark-fg-item-hover) cursor-pointer",
                                                td { class: "px-2 py-2", "{attr_tmpl.name}" }
                                                td { class: "px-2 py-2", "{attr_tmpl.description.as_deref().unwrap_or_default()}" }
                                            }
                                        }
                                    })
                            }
                            if entries.is_empty() {
                                tr {
                                    td { class: "py-4", colspan: "2", " " }
                                }
                            }
                        }
                        tfoot {
                            tr {
                                td {
                                    class: "px-2 py-2 text-xs text-right text-(--fg-item) bg-(--bg) dark:text-(--dark-fg-item) dark:bg-(--dark-bg-l1)",
                                    colspan: "2",
                                    if entries.len() == 1 {
                                        "1 entry"
                                    } else {
                                        "{entries.len()} entries"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

async fn fetch_attribute_templates() -> Vec<AttributeTemplate> {
    //
    match reqwest::Client::new()
        .get("http://localhost:9011/data/templates/attributes")
        .send()
        .await
    {
        Ok(resp) => match resp.json::<Vec<AttributeTemplate>>().await {
            Ok(attr_tmpls) => {
                log::info!(">>> [fetch_attribute_templates] Got {} entries.", attr_tmpls.len());
                attr_tmpls
            }
            Err(e) => {
                log::error!("Failed to parse attribute templates response. Reason: '{}'.", e);
                vec![]
            }
        },
        Err(e) => {
            log::error!("Failed to fetch attribute templates. Reason: '{}'.", e);
            vec![]
        }
    }
}
