use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::result::Error as DieselError;
use std::fmt;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for AppError {}

impl From<AppError> for DieselError {
    fn from(err: AppError) -> Self {
        DieselError::QueryBuilderError(Box::new(err))
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError { message: err.to_string() }
    }
}

/// Hash a password using bcrypt
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

/// Verify a password against a hash using bcrypt
pub fn check_password_hash(password: &str, hash: &str) -> Result<bool, AppError> {
    let valid = verify(password, hash)?;
    Ok(valid)
}

/// Extract the login name from a matric number
pub fn extract_login_name(matric_no: &str) -> Result<String, AppError> {
    let parts: Vec<&str> = matric_no.split('/').collect();
    if !parts.is_empty() {
        return Ok(parts[0].to_lowercase());
    }
    Err(AppError { message: "invalid matric number format".to_string() })
}
