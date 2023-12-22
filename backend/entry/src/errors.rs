pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error{
    // For The config
    EnvMissing(&'static str),
    // For the database
    DbConnection(sqlx::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::EnvMissing(env_var) => write!(f, "Missing environment variable: {}", env_var),
            Self::DbConnection(err) => write!(f, "Database connection error: {}", err),
        }
    }
}