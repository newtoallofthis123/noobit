use super::errors::UtilsError;

pub fn get_password_hash(password: &str) -> Result<String, UtilsError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST).map_err(UtilsError::HashError)
}

pub fn verify_hash(password: &str, hash: &str) -> Result<bool, UtilsError> {
    bcrypt::verify(password, hash).map_err(UtilsError::WrongPassword)
}

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}