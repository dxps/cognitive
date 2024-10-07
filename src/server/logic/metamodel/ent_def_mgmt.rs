use crate::{
    domain::model::{EntityDef, Id},
    server::{create_id, AppResult, EntityDefRepo},
};
use std::{collections::HashMap, sync::Arc};

pub struct EntityDefMgmt {
    ent_repo: Arc<EntityDefRepo>,
}

impl EntityDefMgmt {
    //
    pub fn new(ent_repo: Arc<EntityDefRepo>) -> Self {
        Self { ent_repo }
    }

    pub async fn list_ids_names(&self) -> AppResult<HashMap<Id, String>> {
        let items = self.ent_repo.list_ids_names().await?;
        let result: HashMap<Id, String> = items.into_iter().map(|item| (item.0, item.1)).collect();
        Ok(result)
    }

    pub async fn list(&self) -> AppResult<Vec<EntityDef>> {
        self.ent_repo.list(None).await
    }

    pub async fn add(&self, mut ent_def: EntityDef) -> AppResult<Id> {
        ent_def.id = create_id();
        self.ent_repo.add(&ent_def).await?;
        Ok(ent_def.id)
    }

    pub async fn get(&self, id: &Id) -> Option<EntityDef> {
        self.ent_repo.get(id).await
    }

    pub async fn update(&self, ent_def: EntityDef) -> AppResult<()> {
        self.ent_repo.update(&ent_def).await
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        self.ent_repo.remove(id).await
    }
}
