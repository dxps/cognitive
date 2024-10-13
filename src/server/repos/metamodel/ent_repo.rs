use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::{Entity, Id, IntegerAttribute, ItemType, SmallintAttribute, TextAttribute},
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

    /// List all the entities that are found in the database.
    /// Note that the attributes of the entities are not loaded.
    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> AppResult<Vec<Entity>> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!("SELECT e.id, e.def_id, e.listing_attr_name, e.listing_attr_value, ed.name as kind FROM entities e JOIN entity_defs ed ON e.def_id = ed.id ORDER BY name LIMIT {limit} OFFSET {offset}");

        sqlx::query_as::<_, Entity>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<Entity>> {
        //
        let mut res = None;
        if let Ok(ent_opt) = sqlx::query_as::<_, Entity>(
            "SELECT e.id, e.def_id, e.listing_attr_name, e.listing_attr_value, ed.name as kind FROM entities e JOIN entity_defs ed ON e.def_id = ed.id WHERE e.id = $1",
        )
        .bind(id)
        .fetch_optional(self.dbcp.as_ref())
        .await
        {
            if let Some(mut ent) = ent_opt {
                // Get all the attributes of an entity in one shot.
                let query = "
                    SELECT ad.name, ad.value_type, a.def_id, a.value as text_value, 0 as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad 
                        JOIN text_attributes a ON a.def_id = ad.id  
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, a.value as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN smallint_attributes a ON a.def_id = ad.id
                        WHERE a.owner_type = 'eni' and a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, a.value as integer_value, 0 as bigint_value, 0 as real_value, 
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN integer_attributes a ON a.def_id = ad.id
                        WHERE a.owner_type = 'eni' and a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 as integer_value, a.value as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN bigint_attributes a ON a.def_id = ad.id
                        WHERE a.owner_type = 'eni' and a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, a.value as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN real_attributes a ON a.def_id = ad.id
                        WHERE a.owner_type = 'eni' and a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN boolean_attributes a ON a.def_id = ad.id
                        WHERE a.owner_type = 'eni' and a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, a.value as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN date_attributes a ON a.def_id = ad.id
                        WHERE a.owner_type = 'eni' and a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, a.value as timestamp_value 
                        FROM attribute_defs ad
                        JOIN timestamp_attributes a ON a.def_id = ad.id
                        WHERE a.owner_type = 'eni' and a.owner_id = $1;
                ";
                let rows = sqlx::query(query).bind(id).fetch_all(self.dbcp.as_ref()).await?;
                fill_in_entity_attributes(&mut ent, rows);
                res = Some(ent);
            }
        };
        Ok(res)
    }

    pub async fn add(&self, ent: &Entity) -> AppResult<()> {
        //
        let mut txn = self.dbcp.begin().await?;

        if let Err(e) =
            sqlx::query("INSERT INTO entities (id, def_id, listing_attr_name, listing_attr_value) VALUES ($1, $2, $3, $4)")
                .bind(ent.id.clone())
                .bind(ent.kind.clone())
                .bind(ent.listing_attr_name.clone())
                .bind(ent.listing_attr_value.clone())
                .execute(&mut *txn)
                .await
        {
            txn.rollback().await?;
            log::error!("Failed to add entity. Cause: '{}'.", e);
            return AppResult::Err(e.into());
        }

        for attr in ent.text_attributes.clone() {
            if let Err(e) =
                sqlx::query("INSERT INTO text_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                    .bind(ent.id.clone())
                    .bind(ItemType::Entity.value())
                    .bind(attr.def_id)
                    .bind(attr.value)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity's text attribute: {}", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent.smallint_attributes.clone() {
            if let Err(e) =
                sqlx::query("INSERT INTO smallint_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                    .bind(ent.id.clone())
                    .bind(ItemType::Entity.value())
                    .bind(attr.def_id)
                    .bind(attr.value)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity's smallint attribute: {}", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent.int_attributes.clone() {
            if let Err(e) =
                sqlx::query("INSERT INTO integer_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                    .bind(ent.id.clone())
                    .bind(ItemType::Entity.value())
                    .bind(attr.def_id)
                    .bind(attr.value)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity's integer attribute: {}", e);
                return AppResult::Err(e.into());
            }
        }

        for attr in ent.boolean_attributes.clone() {
            if let Err(e) =
                sqlx::query("INSERT INTO boolean_attributes (owner_id, owner_type, def_id, value) VALUES ($1, $2, $3, $4)")
                    .bind(ent.id.clone())
                    .bind(ItemType::Entity.value())
                    .bind(attr.def_id)
                    .bind(attr.value)
                    .execute(&mut *txn)
                    .await
            {
                txn.rollback().await?;
                log::error!("Failed to add entity's boolean attribute: {}", e);
                return AppResult::Err(e.into());
            }
        }

        txn.commit().await?;

        Ok(())
    }

    pub async fn update(&self, _ent: &Entity) -> AppResult<()> {
        unimplemented!("TODO: Unimplemented")
    }

    pub async fn remove(&self, _id: &Id) -> AppResult<()> {
        unimplemented!("TODO: Unimplemented")
    }
}

impl FromRow<'_, PgRow> for Entity {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            kind: row.get("kind"),
            def_id: row.get("def_id"),
            text_attributes: vec![],
            smallint_attributes: vec![],
            int_attributes: vec![],
            boolean_attributes: vec![],
            listing_attr_name: row.get("listing_attr_name"),
            listing_attr_value: row.get("listing_attr_value"),
        })
    }
}

fn fill_in_entity_attributes(ent: &mut Entity, rows: Vec<PgRow>) {
    //
    for row in rows {
        match row.get::<&str, &str>("value_type") {
            "text" => {
                log::debug!("Found text attribute '{}'.", row.get::<&str, &str>("name"));
                ent.text_attributes.push(TextAttribute::new(
                    row.get("name"),
                    row.get("text_value"),
                    row.get("def_id"),
                    ent.id.clone(),
                    ItemType::Entity,
                ));
            }
            "smallint" => {
                log::debug!("Found smallint attribute '{}'.", row.get::<&str, &str>("name"));
                ent.smallint_attributes.push(SmallintAttribute::new(
                    row.get("name"),
                    row.get("smallint_value"),
                    row.get("def_id"),
                ));
            }
            "integer" => {
                log::debug!("Found integer attribute '{}'.", row.get::<&str, &str>("name"));
                ent.int_attributes.push(IntegerAttribute::new(
                    row.get("name"),
                    row.get("integer_value"),
                    row.get("def_id"),
                ));
            }
            _ => {
                log::debug!("Found attribute '{}'.", row.get::<&str, &str>("name"));
            }
        }
    }
}
