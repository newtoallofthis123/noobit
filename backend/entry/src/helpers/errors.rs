use core::fmt;

#[derive(Debug)]
pub enum UtilsError{
    HashError(bcrypt::BcryptError),
    WrongPassword(bcrypt::BcryptError),
}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UtilsError::HashError(e) => write!(f, "HashError: {}", e),
            UtilsError::WrongPassword(e) => write!(f, "WrongPassword: {}", e),
        }
    }
}