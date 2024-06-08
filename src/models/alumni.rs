use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Alumni {
    pub id: Uuid,
    pub convocation_year: i32,
    pub name: String,
    pub phone: String,
    pub programme: String,
    pub email: String,
    pub matric_no: String,
    pub nationality: String,
    pub address: String,
    pub occupation: String,
    pub location: String,
    pub gender: String,
    pub password: String,
    pub workplace: String,
    pub supervisor: Vec<String>,
    pub thesis_title: String,
    pub initial_registration_session: String,
    pub faculty_id: i32,
    pub is_first_time_login: bool,
    pub is_hidden: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
