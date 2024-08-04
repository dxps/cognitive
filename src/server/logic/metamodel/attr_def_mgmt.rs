use std::sync::Arc;

use crate::{
    domain::model::{AttributeDef, Id},
    server::{AppResult, AttributeDefRepo},
};

pub struct AttributeDefMgmt {
    attr_repo: Arc<AttributeDefRepo>,
}

impl AttributeDefMgmt {
    //
    pub fn new(attr_repo: Arc<AttributeDefRepo>) -> Self {
        Self { attr_repo }
    }

    pub async fn get(&self, id: &String) -> Option<AttributeDef> {
        //
        self.attr_repo.get(id).await
    }

    pub async fn list(&self) -> Vec<AttributeDef> {
        //
        self.attr_repo.list(None).await
    }

    /// Add a new attribute definition. It returns the id of the repository entry.
    pub async fn add(
        &self,
        name: String,
        description: String,
        value_type: String,
        default_value: String,
        is_required: bool,
        is_multivalued: bool,
        tag_id: String,
    ) -> AppResult<Id> {
        //
        self.attr_repo
            .add(
                name,
                description,
                value_type,
                default_value,
                is_required,
                is_multivalued,
                tag_id,
            )
            .await
    }

    /// Update an existing attribute definition.
    pub async fn update(&self, item: AttributeDef) -> AppResult<()> {
        //
        self.attr_repo.update(item).await
    }
}
