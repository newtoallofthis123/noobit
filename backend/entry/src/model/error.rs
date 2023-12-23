#[derive(Debug)]
pub enum ModelError{
    DbConnection(sqlx::Error),
    DbQuery(sqlx::Error),
}

impl core::fmt::Display for ModelError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::DbConnection(err) => write!(f, "Database connection error: {}", err),
            Self::DbQuery(err) => write!(f, "Database query error: {}", err),
        }
    }
}