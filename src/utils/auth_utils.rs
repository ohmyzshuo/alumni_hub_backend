use bcrypt::{hash, verify, DEFAULT_COST};
use std::error::Error;

pub fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn check_password_hash(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    let valid = verify(password, hash)?;
    Ok(valid)
}

pub fn extract_login_name(matric_no: &str) -> Result<String, Box<dyn Error>> {
    let parts: Vec<&str> = matric_no.split('/').collect();
    if !parts.is_empty() {
        return Ok(parts[0].to_lowercase());
    }
    Err("invalid matric number format".into())
}
