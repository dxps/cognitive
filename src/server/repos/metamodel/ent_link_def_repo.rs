use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::{Cardinality, EntityLinkDef, Id},
    server::{AppError, AppResult},
};

pub struct EntityLinkDefRepo {
    pub dbcp: Arc<PgPool>,
}

impl EntityLinkDefRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn list(&self) -> AppResult<Vec<EntityLinkDef>> {
        //
        let query = "SELECT id, name, description, cardinality, source_entity_def_id, target_entity_def_id  
                     FROM entity_link_defs ORDER BY name";
        sqlx::query_as::<_, EntityLinkDef>(query)
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))
            .map_err(|e| AppError::from(e))?
    }

    pub async fn add(&self, item: &EntityLinkDef) -> AppResult<()> {
        //
        let query = "INSERT INTO entity_link_defs (id, name, description, cardinality, source_entity_def_id, target_entity_def_id) VALUES ($1, $2, $3, $4, $5, $6)";
        sqlx::query(query)
            .bind(item.id.as_str())
            .bind(&item.name)
            .bind(&item.description)
            .bind(&item.cardinality.to_string())
            .bind(item.source_entity_def_id.as_str())
            .bind(item.target_entity_def_id.as_str())
            .execute(self.dbcp.as_ref())
            .await
            .map(|_| AppResult::Ok(()))?
    }
}

impl FromRow<'_, PgRow> for EntityLinkDef {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self::new(
            Id::new_from(row.get("id")),
            row.get("name"),
            row.get("description"),
            Cardinality::from(row.get::<&str, &str>("cardinality")),
            Id::new_from(row.get("source_entity_def_id")),
            Id::new_from(row.get("target_entity_def_id")),
        ))
    }
}
