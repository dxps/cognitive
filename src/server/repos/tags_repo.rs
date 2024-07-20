use std::sync::Arc;

use sqlx::{postgres::PgRow, FromRow, PgPool, Row};

use crate::{domain::model::Tag, server::PaginationOpts};

pub struct TagsRepo {
    pub dbcp: Arc<PgPool>,
}

impl TagsRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get(&self, id: String) -> Option<Tag> {
        //
        sqlx::query_as::<_, Tag>("SELECT id, name, description FROM tags WHERE id = $1")
            .bind(id)
            .fetch_one(self.dbcp.as_ref())
            .await
            .ok()
    }

    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> Vec<Tag> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!("SELECT id, name, description FROM tags ORDER BY id LIMIT {limit} OFFSET {offset}");
        log::debug!("Listing tags w/ limit: {}, offset: {}.", limit, offset);

        sqlx::query_as::<_, Tag>(query.as_str()) // FYI: Binding (such as .bind(limit) didn't work, that's why the query.
            .fetch_all(self.dbcp.as_ref())
            .await
            .ok()
            .unwrap_or_default()
    }
}

impl FromRow<'_, PgRow> for Tag {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
        })
    }
}
