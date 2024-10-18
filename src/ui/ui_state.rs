use crate::{
    domain::model::{EntityDef, Id, Tag},
    server::fns::{get_tags, list_entities_defs},
};
use dioxus::signals::{GlobalSignal, Readable};
use std::ops::Deref;
use std::{collections::HashMap, sync::Arc};

pub struct UiState {
    pub app_ready: GlobalSignal<bool>,
    pub tags: GlobalSignal<Arc<HashMap<Id, Tag>>>,
    pub tags_loaded: GlobalSignal<bool>,
    pub ent_defs: GlobalSignal<HashMap<Id, EntityDef>>,
}

impl UiState {
    pub const fn new() -> Self {
        Self {
            app_ready: GlobalSignal::new(|| false),
            tags: GlobalSignal::new(|| Arc::new(HashMap::new())),
            tags_loaded: GlobalSignal::new(|| false),
            ent_defs: GlobalSignal::new(|| HashMap::new()),
        }
    }

    pub async fn get_tags(&self) -> Arc<HashMap<Id, Tag>> {
        if self.tags.read().is_empty() {
            let res = get_tags().await;
            match res {
                Ok(tags) => {
                    let tags_map: HashMap<Id, Tag> = tags.into_iter().map(|tag| (tag.id.clone(), tag)).collect();
                    let tags_map = Arc::new(tags_map);
                    *self.tags.write() = tags_map;
                }
                Err(e) => log::error!(">>> [UiGlobalSignals.get_tags] Failed to get tags: {}", e),
            }
        }
        self.tags.read().clone()
    }

    pub async fn get_tag(&self, id: &Id) -> Option<Tag> {
        if self.tags.read().is_empty() {
            _ = self.get_tags().await;
        }
        self.tags.read().get(id).cloned()
    }

    pub async fn add_tag(&self, tag: Tag) {
        let tags = self.tags.read().clone();
        let mut tags = tags.deref().clone();
        tags.insert(tag.id.clone(), tag);
        *self.tags.write() = Arc::new(tags);
    }

    pub async fn update_tag(&self, tag: Tag) {
        let tags = self.tags.read().clone();
        let updated_tags: HashMap<Id, Tag> = tags
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

    pub async fn remove_tag(&self, id: Id) {
        let tags = self.tags.read().clone();
        let updated_tags: HashMap<Id, Tag> = tags
            .iter()
            .filter(|(_, v)| v.id != id)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        *self.tags.write() = Arc::new(updated_tags);
    }

    /// Get the entities definitions.
    /// If they haven't been loaded yet, it fetches them from the server.
    pub async fn get_ent_defs(&self) -> HashMap<Id, EntityDef> {
        if self.ent_defs.read().is_empty() {
            if let Ok(ent_defs) = list_entities_defs().await {
                log::debug!("[UiGlobals] Got entity defs: {:?}", ent_defs);
                *self.ent_defs.write() = ent_defs.into_iter().map(|def| (def.id.clone(), def)).collect();
            }
        };
        self.ent_defs.read().clone()
    }

    pub async fn get_ent_def(&self, id: &Id) -> Option<EntityDef> {
        if self.ent_defs.read().is_empty() {
            if let Ok(ent_defs) = list_entities_defs().await {
                log::debug!("[UiGlobals] Got entity defs: {:?}", ent_defs);
                *self.ent_defs.write() = ent_defs.into_iter().map(|def| (def.id.clone(), def)).collect();
            }
        };
        self.ent_defs.read().get(id).cloned()
    }

    pub fn get_ent_def_sync(&self, id: &Id) -> Option<EntityDef> {
        self.ent_defs.read().get(id).cloned()
    }
}

pub static UI_STATE: UiState = UiState::new();
