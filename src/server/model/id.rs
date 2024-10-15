use crate::domain::model::Id;
use sqlx::{database::HasValueRef, postgres::PgRow, Decode, Error, FromRow, Postgres, Row};

impl FromRow<'_, PgRow> for Id {
    fn from_row(row: &PgRow) -> Result<Self, Error> {
        Ok(Id::from(row.get::<&str, _>("id")))
    }
}

impl<'r> Decode<'r, Postgres> for Id {
    fn decode(
        value: <Postgres as HasValueRef<'r>>::ValueRef,
    ) -> std::result::Result<Id, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        let id = value.as_str()?;
        Ok(Id::new_from(id.to_string()))
    }
}
