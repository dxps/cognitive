use crate::{
    domain::model::{AttributeDef, AttributeValueType},
    server::PaginationOpts,
};
use sqlx::{postgres::PgRow, FromRow, PgPool, Row};
use std::sync::Arc;

pub struct AttributeDefRepo {
    pub dbcp: Arc<PgPool>,
}

impl AttributeDefRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get(&self, id: String) -> Option<AttributeDef> {
        //
        sqlx::query_as::<_, AttributeDef>(
            "SELECT id, name, description, value_type, default_value, multivalued, composite, required, value_rules, tag_id FROM attribute_defs WHERE id = $1"
        )
            .bind(id)
            .fetch_one(self.dbcp.as_ref())
            .await
            .ok()
    }

    pub async fn list(&self, pagination_opts: Option<&PaginationOpts>) -> Vec<AttributeDef> {
        //
        let default_opts = PaginationOpts::default();
        let pagination_opts = pagination_opts.unwrap_or(&default_opts);
        let limit = pagination_opts.limit.unwrap_or(10);
        let offset = (pagination_opts.page.unwrap_or(1) - 1) * limit;
        let query = format!(
            "SELECT id, name, description, value_type, default_value, multivalued, composite, required, value_rules, tag_id FROM attribute_defs ORDER BY id LIMIT {limit} OFFSET {offset}"
        );
        log::debug!("Listing attribute defs w/ limit: {}, offset: {}.", limit, offset);

        sqlx::query_as::<_, AttributeDef>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .ok()
            .unwrap_or_default()
    }

    pub async fn add(
        &self,
        name: String,
        description: String,
        value_type: String,
        default_value: String,
        is_required: bool,
        is_multivalued: bool,
        tag_id: String,
    ) -> AttributeDef {
        //
        sqlx::query_as::<_, AttributeDef>(
            "INSERT INTO attribute_defs (name, description, value_type, default_value, required, multivalued, tag_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
        )
            .bind(name)
            .bind(description)
            .bind(value_type)
            .bind(default_value)
            .bind(is_required)
            .bind(is_multivalued)
            .bind(tag_id)
            .fetch_one(self.dbcp.as_ref())
            .await
            .ok()
            .unwrap()
    }
}

impl FromRow<'_, PgRow> for AttributeDef {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            value_type: AttributeValueType::from(row.get::<&str, &str>("value_type")),
            default_value: row.get("default_value"),
            is_multivalued: row.get("multivalued"),
            is_composite: row.get("composite"),
            is_required: row.get("required"),
            value_rules: row.get("value_rules"),
            tag: None,
        })
    }
}
