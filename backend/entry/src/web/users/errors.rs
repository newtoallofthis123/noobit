use http::StatusCode;
use std::fmt;

pub enum UserError {
    NonBindedJSON(String),
    WrongPassword(bcrypt::BcryptError),
    HashError(bcrypt::BcryptError),
    QueryProblem(sea_query::error::Error),
    DbConnection(sqlx::Error),
    UserNotFound,
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserError::NonBindedJSON(e) => write!(f, "{}", e),
            UserError::WrongPassword(e) => write!(f, "{}", e),
            UserError::HashError(e) => write!(f, "{}", e),
            UserError::QueryProblem(e) => write!(f, "{}", e),
            UserError::DbConnection(e) => write!(f, "{}", e),
            UserError::UserNotFound => write!(f, "User not found"),
        }
    }
}

pub fn handle_error(e: UserError) -> (String, StatusCode) {
    match e {
        UserError::NonBindedJSON(_) => {
            ("Unable to parse JSON".to_string(), StatusCode::BAD_REQUEST)
        }
        UserError::WrongPassword(_) => (
            "Unable to hash password".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        UserError::HashError(_) => (
            "Unable to hash password".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        UserError::QueryProblem(_) => (
            "Unable to create user".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        UserError::DbConnection(_) => (
            "Unable to create user".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
        UserError::UserNotFound => (
            "Unable to create user".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ),
    }
}
