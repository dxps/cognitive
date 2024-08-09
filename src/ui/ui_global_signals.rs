use std::{collections::HashMap, ops::Deref, sync::Arc};

use crate::{domain::model::Tag, server::fns::get_tags};
use dioxus::signals::{GlobalSignal, Readable};

pub struct UiGlobals {
    pub app_ready: GlobalSignal<bool>,
    pub tags: GlobalSignal<Arc<HashMap<String, Tag>>>,
    pub tags_loaded: GlobalSignal<bool>,
}

impl UiGlobals {
    pub const fn new() -> Self {
        Self {
            app_ready: GlobalSignal::new(|| false),
            tags: GlobalSignal::new(|| Arc::new(HashMap::new())),
            tags_loaded: GlobalSignal::new(|| false),
        }
    }

    pub async fn get_tags(&self) -> Arc<HashMap<String, Tag>> {
        if self.tags.read().is_empty() {
            let res = get_tags().await;
            match res {
                Ok(tags) => {
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

    pub async fn add_tag(&self, tag: Tag) {
        let tags = self.tags.read().clone();
        let mut tags = tags.deref().clone();
        tags.insert(tag.id.clone(), tag);
        *self.tags.write() = Arc::new(tags);
    }

    pub async fn update_tag(&self, tag: Tag) {
        let tags = self.tags.read().clone();
        let updated_tags: HashMap<String, Tag> = tags
            .iter()
            .map(|(k, v)| {
                if v.id == tag.id {
                    (k.clone(), tag.clone())
                } else {
                    (k.clone(), v.clone())
                }
            })
            .collect();
        *self.tags.write() = Arc::new(updated_tags);
    }
}

pub static UI_GLOBALS: UiGlobals = UiGlobals::new();
