use indexmap::IndexMap;
use sqlx::{FromRow, PgPool, Row, postgres::PgRow};
use std::sync::Arc;

pub struct ObjectTemplateRepo {
    pub dbcp: Arc<PgPool>,
}

impl ObjectTemplateRepo {
    //
    pub fn new(dbcp: Arc<PgPool>) -> Self {
        Self { dbcp }
    }
}
