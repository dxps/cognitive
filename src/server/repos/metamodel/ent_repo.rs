use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

use crate::{
    domain::model::{Entity, EntityDef, Id, IntegerAttribute, TextAttribute},
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

    pub async fn add(&self, ent: &Entity) -> AppResult<()> {
        //
        // let mut _txn = self.dbcp.begin().await?;
        unimplemented!("TODO: Unimplemented")
        //Ok(())
    }

    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> AppResult<Vec<Entity>> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!("SELECT e.id, ed.name as kind FROM entities e JOIN entity_defs ed ON e.def_id = ed.id ORDER BY name LIMIT {limit} OFFSET {offset}");

        sqlx::query_as::<_, Entity>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .map(|res| AppResult::Ok(res))?
    }

    pub async fn get(&self, id: &Id) -> AppResult<Option<Entity>> {
        //
        let mut res = None;
        if let Ok(ent_opt) = sqlx::query_as::<_, Entity>(
            "SELECT e.id, ed.name, description FROM entities e JOIN entity_defs ed ON e.def_id = ed.id WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.dbcp.as_ref())
        .await
        {
            if let Some(mut ent) = ent_opt {
                // Get all the attributes of an entity in one shot.
                let query = r#"""
                    SELECT ad.id, ad.name, ad.value_type, a.value as text_value, 0 as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad 
                        JOIN text_attributes a ON a.def_id = ad.id  
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, a.value as smallint_value, 0 as integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN smallint_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, a.value as integer_value, 0 as bigint_value, 0 as real_value, 
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN integer_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 as integer_value, a.value as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN bigint_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, a.value as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN real_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, CURRENT_TIMESTAMP as timestamp_value
                        FROM attribute_defs ad
                        JOIN boolean_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, a.value as date_value, CURRENT_TIMESTAMP as timestamp_value 
                        FROM attribute_defs ad
                        JOIN date_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1
                    UNION ALL 
                    SELECT ad.id, ad.name, ad.value_type, '' as text_value, 0 as smallint_value, 0 integer_value, 0 as bigint_value, 0 as real_value,
                        false as bool_value, CURRENT_DATE as date_value, a.value as timestamp_value 
                        FROM attribute_defs ad
                        JOIN timestamp_attributes a ON a.def_id = ad.id
                        WHERE a.owner_id = $1;
                """#;
                let rows = sqlx::query(query).bind(id).fetch_all(self.dbcp.as_ref()).await?;
                fill_in_entity_attributes(&mut ent, rows);
                res = Some(ent);
            }
        };
        Ok(res)
    }

    pub async fn update(&self, ent: &Entity) -> AppResult<()> {
        unimplemented!("TODO: Unimplemented")
    }

    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        unimplemented!("TODO: Unimplemented")
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
            smallint_attributes: vec![],
        })
    }
}

fn fill_in_entity_attributes(ent: &mut Entity, rows: Vec<PgRow>) {
    //
    for row in rows {
        match row.get::<&str, &str>("value_type") {
            "text" => {
                log::debug!("Found text attribute '{}'.", row.get::<&str, &str>("name"));
                ent.text_attributes
                    .push(TextAttribute::new(row.get("id"), row.get("name"), row.get("text_value")));
            }
            "smallint" => {
                log::debug!("Found smallint attribute '{}'.", row.get::<&str, &str>("name"));
                ent.smallint_attributes
                    .push(IntegerAttribute::new(row.get("name"), row.get("smallint_value")));
            }
            _ => {
                log::debug!("Found attribute '{}'.", row.get::<&str, &str>("name"));
            }
        }
    }
}
