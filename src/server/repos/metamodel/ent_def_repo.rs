use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::EntityDef,
    server::{AppResult, PaginationOpts},
};

pub struct EntityDefRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityDefRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> AppResult<Vec<EntityDef>> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!("SELECT id, name, description FROM entity_defs ORDER BY name LIMIT {limit} OFFSET {offset}");
        log::debug!("Listing tags w/ limit: {}, offset: {}.", limit, offset);

        sqlx::query_as::<_, EntityDef>(query.as_str()) // FYI: Binding (such as .bind(limit) didn't work, that's why the query.
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    pub async fn add(&self, ent_def: &EntityDef) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("INSERT INTO entity_defs (id, name, description) VALUES ($1, $2, $3)")
            .bind(ent_def.id.clone())
            .bind(ent_def.name.clone())
            .bind(ent_def.description.clone())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to add entity def: {}", e);
            return AppResult::Err(e.into());
        }

        for attr_def in ent_def.attributes.clone() {
            if let Err(e) =
                sqlx::query("INSERT INTO entity_defs_attribute_defs_xref (entity_def_id, attribute_def_id) VALUES ($1, $2)")
                    .bind(ent_def.id.clone())
                    .bind(attr_def.id)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity def's attributes: {}", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;

        AppResult::Ok(())
    }
}

impl FromRow<'_, PgRow> for EntityDef {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self::new(row.get("id"), row.get("name"), row.get("description")))
    }
}
