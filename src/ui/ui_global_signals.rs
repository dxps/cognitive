use std::sync::Arc;

use crate::{domain::model::Tag, server::fns::tags::get_tags};
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
                Ok(tags) => *self.tags.write() = Arc::new(tags),
                Err(e) => log::error!(">>> Failed to get tags: {}", e),
            }
        }
        self.tags.read().clone()
    }
}

pub static UI_GLOBAL_SIGNALS: UiGlobalSignals = UiGlobalSignals::new();
