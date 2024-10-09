use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::{AttributeDef, EntityDef, Id},
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

    pub async fn list_ids_names(&self) -> AppResult<Vec<(Id, String)>> {
        //
        let query = "SELECT id, name FROM entity_defs ORDER BY name";
        sqlx::query_as::<_, (Id, String)>(query)
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> AppResult<Vec<EntityDef>> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!("SELECT id, name, description FROM entity_defs ORDER BY name LIMIT {limit} OFFSET {offset}");

        let mut ent_defs = sqlx::query_as::<_, EntityDef>(query.as_str()) // FYI: Binding (such as .bind(limit) didn't work, that's why the query.
            .fetch_all(self.dbcp.as_ref())
            .await?;

        for ent_def in &mut ent_defs {
            if let Ok(attrs) = sqlx::query_as::<_, AttributeDef>(
                "SELECT id, name, description, value_type, default_value, required, multivalued, tag_id
             FROM attribute_defs ad JOIN entity_defs_attribute_defs_xref ed_ad_xref
             ON ad.id = ed_ad_xref.attribute_def_id where ed_ad_xref.entity_def_id = $1",
            )
            .bind(&ent_def.id)
            .fetch_all(self.dbcp.as_ref())
            .await
            {
                ent_def.attributes = attrs;
            }
        }

        Ok(ent_defs)
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
                log::error!("Failed to add entity def's attribute defs: {}", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;

        AppResult::Ok(())
    }

    pub async fn get(&self, id: &Id) -> Option<EntityDef> {
        //
        let mut res = None;
        if let Ok(res_opt) = sqlx::query_as::<_, EntityDef>("SELECT id, name, description FROM entity_defs WHERE id = $1")
            .bind(id)
            .fetch_optional(self.dbcp.as_ref())
            .await
        {
            if let Some(mut ent_def) = res_opt {
                if let Ok(attrs) = sqlx::query_as::<_, AttributeDef>(
                    "SELECT id, name, description, value_type, default_value, required, multivalued, tag_id 
                     FROM attribute_defs ad JOIN entity_defs_attribute_defs_xref ed_ad_xref 
                     ON ad.id = ed_ad_xref.attribute_def_id where ed_ad_xref.entity_def_id = $1",
                )
                .bind(id)
                .fetch_all(self.dbcp.as_ref())
                .await
                {
                    ent_def.attributes = attrs;
                    res = Some(ent_def);
                }
            }
        };
        res
    }

    pub async fn update(&self, ent_def: &EntityDef) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;
        if let Err(e) = sqlx::query("UPDATE entity_defs SET name = $1, description = $2 WHERE id = $3")
            .bind(ent_def.name.clone())
            .bind(ent_def.description.clone())
            .bind(ent_def.id.clone())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to update entity def: {}", e);
            return AppResult::Err(e.into());
        }

        if let Err(e) = sqlx::query("DELETE FROM entity_defs_attribute_defs_xref WHERE entity_def_id = $1")
            .bind(&ent_def.id)
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to delete entity def's (id:{}) attribute def id: {}", ent_def.id, e);
            return AppResult::Err(e.into());
        }

        for attr_def in ent_def.attributes.clone() {
            if let Err(e) =
                sqlx::query("INSERT INTO entity_defs_attribute_defs_xref (entity_def_id, attribute_def_id) VALUES ($1, $2)")
                    .bind(&ent_def.id)
                    .bind(&attr_def.id)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to update entity def's attribute defs: {}", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;
        AppResult::Ok(())
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        //
        log::debug!(">>> Deleting entity def: {:?}", id);
        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("DELETE FROM entity_defs_attribute_defs_xref WHERE entity_def_id = $1")
            .bind(id)
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to delete entity def attribute def xref: {}", e);
            return AppResult::Err(e.into());
        }

        if let Err(e) = sqlx::query("DELETE FROM entity_defs WHERE id = $1")
            .bind(id)
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to delete entity def: {}", e);
            return AppResult::Err(e.into());
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
