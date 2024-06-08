use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::staffs;
use chrono::NaiveDateTime;
use crate::models::gender::Gender;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Insertable)]
#[table_name = "staffs"]
pub struct Staff {
    pub id: i32,
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_super_admin: Option<bool>,
    pub faculty_id: Option<i32>,
    pub phone: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_hidden: Option<bool>,
    pub gender: Option<Gender>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "staffs"]
pub struct NewStaff {
    pub name: String,
    pub email: String,
    pub is_super_admin: bool,
    pub faculty_id: i32,
    pub phone: String,
    pub username: String,
    pub password: String,
    pub is_hidden: bool,
    pub gender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset)]
#[table_name = "staffs"]
pub struct UpdateStaff {
    pub name: Option<String>,
    pub email: Option<String>,
    pub is_super_admin: Option<bool>,
    pub faculty_id: Option<i32>,
    pub phone: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_hidden: Option<bool>,
    pub gender: Option<String>,
}
