use crate::{
    domain::model::{BooleanAttribute, Entity, EntityLink, Id, IntegerAttribute, ItemType, SmallintAttribute, TextAttribute},
    server::{AppResult, PaginationOpts},
};
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

pub struct EntityLinkRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityLinkRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    /// List all the entities.<br/>
    /// Note that the attributes of the entities are not loaded.
    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> AppResult<Vec<EntityLink>> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!(
            "SELECT el.id, el.def_id, el.source_entity_id, el.target_entity_id, eld.name as kind 
             FROM entity_links el 
             JOIN entity_link_defs eld 
             ON el.def_id = eld.id ORDER BY name LIMIT {limit} OFFSET {offset}"
        );

        sqlx::query_as::<_, EntityLink>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }
}

impl FromRow<'_, PgRow> for EntityLink {
    fn from_row(row: &PgRow) -> sqlx::Result<Self> {
        Ok(EntityLink {
            id: Id::new_from(row.get("id")),
            kind: row.try_get("kind")?,
            def_id: Id::new_from(row.get("def_id")),
            source_entity_id: Id::new_from(row.get("source_entity_id")),
            target_entity_id: Id::new_from(row.get("target_entity_id")),

            text_attributes: vec![],
            smallint_attributes: vec![],
            int_attributes: vec![],
            boolean_attributes: vec![],
        })
    }
}
