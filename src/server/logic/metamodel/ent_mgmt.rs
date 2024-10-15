use crate::{
    domain::model::{Entity, Id},
    server::{AppResult, EntityRepo},
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
        ent.id = Id::new();
        self.set_listing_attr_value(&mut ent).await?;
        self.repo.add(&ent).await?;
        Ok(ent.id)
    }

    async fn set_listing_attr_value(&self, ent: &mut Entity) -> AppResult<()> {
        //
        for attr in ent.text_attributes.clone() {
            if attr.def_id == ent.listing_attr_def_id {
                ent.listing_attr_value = attr.value;
                break;
            }
        }
        Ok(())
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<Entity>> {
        self.repo.get(id).await
    }

    pub async fn update(&self, ent: &Entity) -> AppResult<()> {
        self.repo.update(ent).await
    }

    pub async fn update_listing_addr_name(&self, def_id: &Id, attr_id: &Id) -> AppResult<()> {
        self.repo.update_listing_attr_name_value_by_ent_def_id(def_id, attr_id).await
    }

    pub async fn update_listing_attr_name_by_attr_def_id(&self, attr_def_id: &Id, attr_name: &String) -> AppResult<()> {
        self.repo
            .update_listing_attr_name_by_attr_def_id(attr_def_id, attr_name)
            .await
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        self.repo.remove(id).await
    }
}
