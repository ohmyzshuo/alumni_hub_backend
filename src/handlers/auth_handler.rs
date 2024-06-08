use crate::schema::{alumnis, staffs};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use jsonwebtoken::{encode, Header, EncodingKey, errors::Error as JwtError};
use uuid::Uuid;
use chrono::{Duration, Utc, NaiveDateTime};
use serde::{Serialize, Deserialize};
use bcrypt::{verify, hash, DEFAULT_COST};
use std::env;
use async_trait::async_trait;
use crate::models::alumni::Alumni;
use crate::models::staff::Staff;
use crate::utils::auth_utils::{check_password_hash, extract_login_name};

#[derive(Serialize, Deserialize)]
struct Claims {
    user_id: Uuid,
    role: String,
    exp: usize,
}

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct AuthService {
    secret: String,
    pool: DbPool,
}

impl AuthService {
    pub fn new(pool: DbPool) -> Self {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set in the environment variables");
        Self { secret, pool }
    }

    pub async fn authenticate_user(
        &self,
        username: &str,
        password: &str,
        role: &str,
    ) -> Result<(i32, String), diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        if role == "alumni" {
            let login_name = extract_login_name(username)?;

            let alumnis = alumnis::dsl::alumnis;
            let alumnus: Alumni = alumnis
                .filter(alumnis::dsl::matric_no.ilike(format!("{}%", login_name)))
                .first(&mut conn)?;

            if check_password_hash(password, alumnus.password.as_deref().unwrap_or(""))? {
                return Ok((alumnus.id, "alumni".to_string()));
            }
        } else if role == "staff" {
            let staffs = staffs::dsl::staffs;
            let staff: Staff = staffs
                .filter(staffs::dsl::username.eq(username.to_lowercase()))
                .first(&mut conn)?;

            if check_password_hash(password, staff.password.as_deref().unwrap_or(""))? {
                return Ok((staff.id, "admin".to_string()));
            }
        }

        Err(diesel::result::Error::NotFound)
    }

}
