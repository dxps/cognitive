use serde::Deserialize;

use crate::{domain::model::Tag, server::TagsRepo};
use std::sync::Arc;

#[derive(Clone)]
pub struct TagMgmt {
    tag_repo: Arc<TagsRepo>,
}

impl TagMgmt {
    //
    pub fn new(tag_repo: Arc<TagsRepo>) -> Self {
        Self { tag_repo }
    }

    pub async fn get(&self, id: String) -> Option<Tag> {
        //
        self.tag_repo.get(id).await
    }

    pub async fn list(&self) -> Vec<Tag> {
        //
        self.tag_repo.list(None).await
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct PaginationOpts {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}
