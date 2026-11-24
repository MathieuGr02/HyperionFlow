use crate::database_type::DatabaseType;

/// The postgresql database.
/// Implements the [`DatabaseType`] trait holding general database information.
pub struct PostgreSQL;
impl DatabaseType for PostgreSQL {
    const NAME: &str = "PostgreSQL";
    const PLACEHOLDER: &str = "$";
}
