use crate::{domain::model::new_id, infra::AttributeTemplateRepo};
use shlib::{
    AppResult,
    domain::model::{AttributeTemplate, Id},
};
use std::sync::Arc;

/// The business logic for (management of) attribute templates.
pub struct AttributeTemplateMgmt {
    pub repo: Arc<AttributeTemplateRepo>,
}

impl AttributeTemplateMgmt {
    //
    pub fn new(repo: Arc<AttributeTemplateRepo>) -> Self {
        Self { repo }
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<AttributeTemplate>> {
        //
        self.repo.get(id).await
    }

    pub async fn list(&self) -> AppResult<Vec<AttributeTemplate>> {
        //
        self.repo.list(None).await
    }

    /// Add a new attribute definition. It returns the id of the stored entry.
    pub async fn add(&self, mut item: AttributeTemplate) -> AppResult<Id> {
        //
        let id = new_id();
        log::debug!("Adding {:?} ...", item);
        item.id = id.clone();
        self.repo.add(&item).await.map(|_| id)
    }

    /// Update an existing attribute definition.
    pub async fn update(&self, item: &AttributeTemplate) -> AppResult<()> {
        //
        self.repo.update(item).await
    }

    /// Remove an existing attribute definition.
    pub async fn remove(&self, id: Id) -> AppResult<()> {
        //
        self.repo.remove(&id).await
    }
}
