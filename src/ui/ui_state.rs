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
    /// the ordered list of tags
    pub tags_list: GlobalSignal<Arc<Vec<Tag>>>,
    pub tags_loaded: GlobalSignal<bool>,
    pub ent_defs_list: GlobalSignal<Vec<EntityDef>>,
}

impl UiState {
    pub const fn new() -> Self {
        Self {
            app_ready: GlobalSignal::new(|| false),
            tags: GlobalSignal::new(|| Arc::new(HashMap::new())),
            tags_list: GlobalSignal::new(|| Arc::new(Vec::new())),
            tags_loaded: GlobalSignal::new(|| false),
            ent_defs_list: GlobalSignal::new(|| Vec::new()),
        }
    }

    pub async fn get_tags(&self) -> Arc<HashMap<Id, Tag>> {
        if self.tags.read().is_empty() {
            let res = get_tags().await;
            match res {
                Ok(tags) => {
                    *self.tags_list.write() = Arc::new(tags.clone());
                    let tags_map: HashMap<Id, Tag> = tags.into_iter().map(|tag| (tag.id.clone(), tag)).collect();
                    let tags_map = Arc::new(tags_map);
                    *self.tags.write() = tags_map;
                }
                Err(e) => log::error!(">>> [UiState.get_tags] Failed to get tags: {}", e),
            }
        }
        self.tags.read().clone()
    }

    pub async fn get_tags_list(&self) -> Arc<Vec<Tag>> {
        if self.tags_list.read().is_empty() {
            _ = self.get_tags().await;
        }
        self.tags_list.read().clone()
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
        tags.insert(tag.id.clone(), tag.clone());
        *self.tags.write() = Arc::new(tags);
        let tags_list = self.tags_list.read().clone();
        let mut tags_list = tags_list.deref().clone();
        tags_list.push(tag);
        *self.tags_list.write() = Arc::new(tags_list);
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

        let tags_list = self.tags_list.read().clone();
        let updated_tags_list: Vec<Tag> = tags_list
            .iter()
            .map(|t| {
                if t.id == tag.id {
                    Tag::new(t.id.clone(), tag.name.clone(), tag.description.clone())
                } else {
                    t.clone()
                }
            })
            .collect();
        *self.tags_list.write() = Arc::new(updated_tags_list);
    }

    pub async fn remove_tag(&self, id: Id) {
        let tags = self.tags.read().clone();
        let updated_tags: HashMap<Id, Tag> = tags
            .iter()
            .filter(|(_, v)| v.id != id)
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        log::debug!("[UiState.remove_tag] Updated tags: {:?}", updated_tags);
        *self.tags.write() = Arc::new(updated_tags);

        let tags_list = self.tags_list.read().clone();
        let updated_tags_list: Vec<Tag> = tags_list.iter().filter(|t| t.id != id).map(|t| t.clone()).collect();
        *self.tags_list.write() = Arc::new(updated_tags_list);
    }

    /// Get the entities definitions.
    /// If they haven't been loaded yet, it fetches them from the server.
    pub async fn get_ent_defs_list(&self) -> Vec<EntityDef> {
        if self.ent_defs_list.read().is_empty() {
            self.get_ent_defs_from_server().await;
        };
        self.ent_defs_list.read().clone()
    }

    pub async fn get_ent_def(&self, id: &Id) -> Option<EntityDef> {
        if self.ent_defs_list.read().is_empty() {
            self.get_ent_defs_from_server().await;
        };
        self.get_ent_def_sync(id)
    }

    async fn get_ent_defs_from_server(&self) {
        match list_entities_defs().await {
            Ok(ent_defs) => {
                log::debug!("[UiState.get_ent_defs_from_server] Got entity defs: {:?}", ent_defs);
                *self.ent_defs_list.write() = ent_defs;
            }
            Err(e) => {
                log::error!("[UiState.get_ent_defs_from_server] Failed to fetch entity defs. Cause: '{e}'.");
            }
        }
    }

    pub fn get_ent_def_sync(&self, id: &Id) -> Option<EntityDef> {
        self.ent_defs_list
            .read()
            .iter()
            .find(|item| item.id == *id)
            .map(|item| item.clone())
    }

    pub fn add_ent_def(&self, ent_def: EntityDef) {
        let mut ent_defs = self.ent_defs_list.read().clone();
        log::debug!(
            "[UiState.add_ent_def] Adding ent_def: {:?} to existing ent_defs: {:?}",
            ent_def,
            ent_defs
        );
        ent_defs.push(ent_def);
        *self.ent_defs_list.write() = ent_defs;
    }

    pub fn update_ent_def(&self, ent_def: EntityDef) {
        let mut ent_defs = self.ent_defs_list.read().clone();
        ent_defs.retain(|ed| ed.id != ent_def.id);
        ent_defs.push(ent_def);
        *self.ent_defs_list.write() = ent_defs;
    }

    pub fn remove_ent_def(&self, id: &Id) {
        let mut ent_defs = self.ent_defs_list.read().clone();
        ent_defs.retain(|ent_def| ent_def.id != *id);
        *self.ent_defs_list.write() = ent_defs;
    }
}

pub static UI_STATE: UiState = UiState::new();
