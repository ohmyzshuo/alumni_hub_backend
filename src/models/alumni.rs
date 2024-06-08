use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::models::gender::Gender;
use crate::schema::alumnis;
use crate::utils::naive_date_time::serialize;
use crate::utils::naive_date_time;
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "alumnis"]
pub struct Alumni {
    pub id: i32,
    pub convocation_year: Option<i32>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub programme: Option<String>,
    pub email: Option<String>,
    pub matric_no: Option<String>,
    pub nationality: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub address: Option<String>,
    pub occupation: Option<String>,
    pub location: Option<String>,
    pub gender: Option<Gender>,
    pub password: Option<String>,
    pub workplace: Option<String>,
    pub supervisor: Option<Vec<String>>,
    pub thesis_title: Option<String>,
    pub initial_registration_session: Option<String>,
    pub faculty_id: Option<i32>,
    pub is_first_time_login: Option<bool>,
    pub is_hidden: Option<bool>,
    #[serde(with = "naive_date_time")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(with = "naive_date_time")]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "alumnis"]
pub struct NewAlumni {
    pub convocation_year: Option<i32>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub programme: Option<String>,
    pub email: Option<String>,
    pub matric_no: Option<String>,
    pub nationality: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub address: Option<String>,
    pub occupation: Option<String>,
    pub location: Option<String>,
    pub gender: Option<Gender>,
    pub password: Option<String>,
    pub workplace: Option<String>,
    pub supervisor: Option<Vec<String>>,
    pub thesis_title: Option<String>,
    pub initial_registration_session: Option<String>,
    pub faculty_id: Option<i32>,
    pub is_first_time_login: Option<bool>,
    pub is_hidden: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[table_name = "alumnis"]
pub struct UpdateAlumni {
    pub convocation_year: Option<i32>,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub programme: Option<String>,
    pub email: Option<String>,
    pub matric_no: Option<String>,
    pub nationality: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub address: Option<String>,
    pub occupation: Option<String>,
    pub location: Option<String>,
    pub gender: Option<Gender>,
    pub password: Option<String>,
    pub workplace: Option<String>,
    pub supervisor: Option<Vec<String>>,
    pub thesis_title: Option<String>,
    pub initial_registration_session: Option<String>,
    pub faculty_id: Option<i32>,
    pub is_first_time_login: Option<bool>,
    pub is_hidden: Option<bool>,
}

