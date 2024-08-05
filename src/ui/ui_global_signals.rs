use std::sync::Arc;

use crate::{domain::model::Tag, server::fns::get_tags};
use dioxus::signals::{GlobalSignal, Readable};

pub struct UiGlobalSignals {
    pub app_ready: GlobalSignal<bool>,
    pub tags: GlobalSignal<Arc<Vec<Tag>>>,
}

impl UiGlobalSignals {
    pub const fn new() -> Self {
        Self {
            app_ready: GlobalSignal::new(|| false),
            tags: GlobalSignal::new(|| Arc::new(vec![])),
        }
    }

    pub async fn get_tags(&self) -> Arc<Vec<Tag>> {
        if self.tags.read().is_empty() {
            let res = get_tags().await;
            match res {
                Ok(tags) => {
                    log::debug!(">>> [UiGlobalSignals.get_tags] Got tags: {:?}", tags);
                    let tags = Arc::new(tags);
                    *self.tags.write() = tags.clone();
                    return tags.clone();
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
        self.tags.read().iter().find(|t| t.id == id).cloned()
    }
}

pub static UI_GLOBAL_SIGNALS: UiGlobalSignals = UiGlobalSignals::new();
