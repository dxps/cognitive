use crate::infra::repos::consts::PG_FK_VIOLATION_CODE;
use shlib::domain::model::AttributeValueType;
use shlib::{
    AppError, AppResult, Pagination,
    domain::model::{AttributeTemplate, Id},
};
use sqlx::Row;
use sqlx::{FromRow, PgPool, postgres::PgRow};
use std::sync::Arc;

pub struct AttributeTemplateRepo {
    pub dbcp: Arc<PgPool>,
}

impl AttributeTemplateRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }

    pub async fn get(&self, id: &Id) -> Option<AttributeTemplate> {
        //
        let item = sqlx::query_as::<_, AttributeTemplatePg>(
            "SELECT id, name, description, value_type, default_value, required, tag_id 
             FROM attribute_defs WHERE id = $1",
        )
        .bind(id.as_str())
        .fetch_one(self.dbcp.as_ref())
        .await
        .ok();

        item.map(|r| r.into())
    }

    pub async fn list(&self, pagination_opts: Option<&Pagination>) -> Vec<AttributeTemplate> {
        //
        let (offset, limit) = Pagination::from(pagination_opts).get_offset_limit();
        let query = format!(
            "SELECT id, name, description, value_type, default_value, required, tag_id 
             FROM attribute_defs ORDER BY name LIMIT {limit} OFFSET {offset}"
        );
        log::debug!("Listing attribute defs w/ limit: {}, offset: {}.", limit, offset);

        let items = sqlx::query_as::<_, AttributeTemplatePg>(query.as_str())
            .fetch_all(self.dbcp.as_ref())
            .await
            .ok()
            .unwrap_or_default();

        items.into_iter().map(|r| r.into()).collect()
    }

    /// Add a new attribute template. It returns the id of the repository entry.
    pub async fn add(&self, item: &AttributeTemplate) -> AppResult<()> {
        //
        sqlx::query(
            "INSERT INTO attribute_defs (id, name, description, value_type, default_value, required, tag_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(&item.id.as_str())
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.value_type.to_string())
        .bind(&item.default_value)
        .bind(item.is_required)
        .execute(self.dbcp.as_ref())
        .await
        .map(|_| Ok(()))
        .map_err(|e| {
            if e.to_string().contains("name_desc_unique") {
                AppError::NameDescriptionNotUnique
            } else {
                log::error!("Failed to add attribute template. Reason: '{}'.", e);
                AppError::Err("An internal error occurred.".into())
            }
        })?
    }

    /// Edit an existing attribute template.
    pub async fn update(&self, item: &AttributeTemplate) -> AppResult<()> {
        //
        sqlx::query(
            "UPDATE attribute_defs 
             SET name=$2, description=$3, value_type=$4, default_value=$5, required=$6
             WHERE id = $1",
        )
        .bind(&item.id.as_str())
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.value_type.to_string())
        .bind(&item.default_value)
        .bind(item.is_required)
        .execute(self.dbcp.as_ref())
        .await
        .map(|_| Ok(()))
        .map_err(|e| {
            if e.to_string().contains("name_desc_unique") {
                AppError::NameDescriptionNotUnique
            } else {
                log::error!("Failed to update attribute template. Reason: '{}'.", e);
                AppError::Err("An internal error occurred.".into())
            }
        })?
    }

    /// Remove (delete) an existing attribute template.
    pub async fn remove(&self, id: &Id) -> AppResult<()> {
        //
        match sqlx::query("DELETE FROM attribute_tmpls WHERE id = $1")
            .bind(id.as_str())
            .execute(self.dbcp.as_ref())
            .await
        {
            Ok(_) => AppResult::Ok(()),
            Err(e) => {
                if let Some(db_err) = e.as_database_error() {
                    if let Some(db_err_code) = db_err.code() {
                        if db_err_code.as_ref() == PG_FK_VIOLATION_CODE {
                            return AppResult::Err(AppError::Err(
                                "Cannot delete attribute template because it is included in the following entity templates:".to_string(),
                            ));
                        }
                    }
                }
                log::error!("Failed to delete entry: {}", e);
                AppResult::Err(AppError::InternalErr("An internal error occurred.".into()))
            }
        }
    }
}

/// To mitigate the 'orphan rule', we implement the `FromRow` trait for our (back-end) AttributeTemplatePg struct,
/// since `AttributeTemplate` is defined outside of this crate.
#[derive(Clone, Debug)]
pub struct AttributeTemplatePg(pub AttributeTemplate);

impl FromRow<'_, PgRow> for AttributeTemplatePg {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let at = AttributeTemplate {
            id: Id::new_from(row.get("id")),
            name: row.get("name"),
            description: row.get("description"),
            value_type: AttributeValueType::from(row.get::<&str, &str>("value_type")),
            default_value: row.get("default_value"),
            is_required: row.get("required"),
        };
        Ok(Self(at))
    }
}

impl From<AttributeTemplatePg> for AttributeTemplate {
    fn from(pg: AttributeTemplatePg) -> Self {
        pg.0
    }
}
