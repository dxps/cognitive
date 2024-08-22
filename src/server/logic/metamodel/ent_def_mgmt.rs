use crate::{
    domain::model::{EntityDef, Id},
    server::{create_id, AppResult, EntityDefRepo},
};
use std::sync::Arc;

pub struct EntityDefMgmt {
    ent_repo: Arc<EntityDefRepo>,
}

impl EntityDefMgmt {
    //
    pub fn new(ent_repo: Arc<EntityDefRepo>) -> Self {
        Self { ent_repo }
    }

    pub async fn list(&self) -> AppResult<Vec<EntityDef>> {
        self.ent_repo.list(None).await
    }

    pub async fn add(&self, mut ent_def: EntityDef) -> AppResult<Id> {
        ent_def.id = create_id();
        self.ent_repo.add(&ent_def).await?;
        Ok(ent_def.id)
    }
}
