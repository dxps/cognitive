use crate::{
    domain::model::{EntityLink, Id, ItemType},
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

    pub async fn add(&self, ent_link: &EntityLink) -> AppResult<()> {
        //
        log::debug!("Adding entity link: '{:?}'.", ent_link);

        let mut txn = self.dbcp.begin().await?;

        if let Err(e) = sqlx::query("INSERT INTO entity_links (id, def_id, source_entity_id, target_entity_id) VALUES ($1, $2, $3, $4)")
            .bind(&ent_link.id.as_str())
            .bind(&ent_link.def_id.as_str())
            .bind(&ent_link.source_entity_id.as_str())
            .bind(&ent_link.target_entity_id.as_str())
            .execute(&mut *txn)
            .await
        {
            txn.rollback().await?;
            log::error!("Failed to add entity link. Cause: '{}'.", e);
            return AppResult::Err(e.into());
        }

        for attr in ent_link.text_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO text_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(&ent_link.id.as_str())
                .bind(ItemType::EntityLink.value())
                .bind(&attr.def_id.as_str())
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!(
                    "Failed to add entity link text attribute w/ owner_id: '{}' owner_type: '{}' def_id:'{}' value:'{}'. Reason: '{}'.",
                    &ent_link.id.as_str(),
                    ItemType::EntityLink.value(),
                    &attr.def_id.as_str(),
                    &attr.value,
                    e
                );
                return AppResult::Err(e.into());
            }
        }

        for attr in ent_link.smallint_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO smallint_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(&ent_link.id)
                .bind(ItemType::EntityLink.value())
                .bind(&attr.def_id)
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity link smallint attribute. Cause: '{}'.", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent_link.int_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO int_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(&ent_link.id)
                .bind(ItemType::EntityLink.value())
                .bind(&attr.def_id)
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity link int attribute. Cause: '{}'.", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent_link.boolean_attributes.iter() {
            if let Err(e) = sqlx::query("INSERT INTO boolean_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                .bind(&ent_link.id)
                .bind(ItemType::EntityLink.value())
                .bind(&attr.def_id)
                .bind(&attr.value)
                .execute(&mut *txn)
                .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity link boolean attribute. Cause: '{}'.", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;

        Ok(())
    }
}

impl FromRow<'_, PgRow> for EntityLink {
    //
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
