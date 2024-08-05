use std::{collections::HashMap, sync::Arc};

use crate::{domain::model::Tag, server::fns::get_tags};
use dioxus::signals::{GlobalSignal, Readable};

pub struct UiGlobalSignals {
    pub app_ready: GlobalSignal<bool>,
    pub tags: GlobalSignal<Arc<HashMap<String, Tag>>>,
}

impl UiGlobalSignals {
    pub const fn new() -> Self {
        Self {
            app_ready: GlobalSignal::new(|| false),
            tags: GlobalSignal::new(|| Arc::new(HashMap::new())),
        }
    }

    pub async fn get_tags(&self) -> Arc<HashMap<String, Tag>> {
        if self.tags.read().is_empty() {
            let res = get_tags().await;
            match res {
                Ok(tags) => {
                    log::debug!(">>> [UiGlobalSignals.get_tags] Got tags: {:?}", tags);
                    let tags_map: HashMap<String, Tag> = tags.into_iter().map(|tag| (tag.id.clone(), tag)).collect();
                    let tags_map = Arc::new(tags_map);
                    *self.tags.write() = tags_map;
                }
                Err(e) => log::error!(">>> [UiGlobalSignals.get_tags] Failed to get tags: {}", e),
            }
        }
        self.tags.read().clone()
    }

    pub async fn get_tag(&self, id: String) -> Option<Tag> {
        if self.tags.read().is_empty() {
            _ = self.get_tags().await;
        }
        self.tags.read().get(&id).cloned()
    }
}

pub static UI_GLOBAL_SIGNALS: UiGlobalSignals = UiGlobalSignals::new();
