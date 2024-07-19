use std::sync::Arc;

use sqlx::{postgres::PgRow, FromRow, PgPool, Row};

use crate::domain::model::Tag;

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
