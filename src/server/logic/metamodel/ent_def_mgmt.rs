use crate::{
    domain::model::EntityDef,
    server::{AppResult, EntityDefRepo},
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
}
