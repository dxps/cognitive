use crate::{
    domain::model::{EntityLink, Id},
    server::{AppResult, EntityLinkRepo},
};
use std::sync::Arc;

pub struct EntityLinkMgmt {
    repo: Arc<EntityLinkRepo>,
}

impl EntityLinkMgmt {
    //
    pub fn new(repo: Arc<EntityLinkRepo>) -> Self {
        Self { repo }
    }

    pub async fn list(&self) -> AppResult<Vec<EntityLink>> {
        self.repo.list(None).await
    }

    pub async fn add(&self, item: EntityLink) -> AppResult<Id> {
        self.repo.add(&item).await
    }
}
