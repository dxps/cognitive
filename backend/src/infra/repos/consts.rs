/// 23503 is postgres specific code for dependencies (named "foreign_key_violation").
/// See: https://www.postgresql.org/docs/16/errcodes-appendix.htm
pub const PG_FK_VIOLATION_CODE: &str = "23503";
