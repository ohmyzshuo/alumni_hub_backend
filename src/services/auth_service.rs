use sqlx::PgPool;
use jsonwebtoken::{encode, Header, EncodingKey, errors::Error as JwtError};
use std::env;
use uuid::Uuid;
use std::sync::Arc;
use crate::models::{alumni, staff};
use crate::utils::{check_password_hash, extract_login_name};
use chrono::{Duration, Utc};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,
    role: String,
    exp: usize,
}

pub struct AuthService {
    secret: String,
}

impl AuthService {
    pub fn new() -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment variables");
        Self { secret }
    }

    pub async fn authenticate_user(
        &self,
        pool: &PgPool,
        username: &str,
        password: &str,
        role: &str,
    ) -> Result<(Uuid, String), sqlx::Error> {
        if role == "alumni" {
            let login_name = extract_login_name(username)?;

            let alumnus: Alumni = sqlx::query_as!(
                Alumni,
                "SELECT * FROM alumni WHERE LOWER(matric_no) LIKE $1 || '/%'",
                login_name
            )
                .fetch_one(pool)
                .await?;

            if check_password_hash(password, &alumnus.password)? {
                return Ok((alumnus.id, "alumni".to_string()));
            }
        } else if role == "staff" {
            let staff: Staff = sqlx::query_as!(
                Staff,
                "SELECT * FROM staff WHERE LOWER(username) = $1",
                username.to_lowercase()
            )
                .fetch_one(pool)
                .await?;

            if check_password_hash(password, &staff.password)? {
                return Ok((staff.id, "admin".to_string()));
            }
        }

        Err(sqlx::Error::RowNotFound)
    }

    pub fn generate_token(&self, user_id: Uuid, role: String) -> Result<String, JwtError> {
        let claims = Claims {
            user_id,
            role,
            exp: (Utc::now() + Duration::hours(3)).timestamp() as usize,
        };
        encode(&Header::default(), &claims, &EncodingKey::from_secret(self.secret.as_ref()))
    }
}

// Optional: Add functions to parse token if needed
