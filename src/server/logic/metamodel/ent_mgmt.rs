use crate::{
    domain::model::{Entity, Id},
    server::{create_id, AppResult, EntityRepo},
};
use std::sync::Arc;

pub struct EntityMgmt {
    repo: Arc<EntityRepo>,
}

impl EntityMgmt {
    //
    pub fn new(repo: Arc<EntityRepo>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> AppResult<Vec<Entity>> {
        self.repo.list(None).await
    }

    pub async fn add(&self, mut ent: Entity) -> AppResult<Id> {
        ent.id = create_id();
        self.repo.add(&ent).await?;
        Ok(ent.id)
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<Entity>> {
        self.repo.get(id).await
    }

    pub async fn update(&self, ent: Entity) -> AppResult<()> {
        self.repo.update(&ent).await
    }

    pub async fn update_listing_addr_name(&self, def_id: Id, attr_id: String) -> AppResult<()> {
        self.repo.update_listing_attr_name_value(def_id, attr_id).await
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        self.repo.remove(id).await
    }
}
