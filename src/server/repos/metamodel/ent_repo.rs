use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::{AttributeDef, Entity, EntityDef, Id},
    server::{AppResult, PaginationOpts},
};

pub struct EntityRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> AppResult<Vec<Entity>> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!("SELECT e.id, ed.name as kind from entities e join entity_defs ed on e.def_id = ed.id ORDER BY name LIMIT {limit} OFFSET {offset}");

        sqlx::query_as::<_, Entity>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }
}

impl FromRow<'_, PgRow> for Entity {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            kind: row.get("kind"),
            def: EntityDef::default(),
            text_attributes: vec![],
            boolean_attributes: vec![],
        })
    }
}
