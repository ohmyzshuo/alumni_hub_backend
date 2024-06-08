use diesel::prelude::*;
use serde::{ Deserialize, Serialize };

#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Staff {
    pub id: i32,
    pub name: String,
    pub position: String,
    pub department: String,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = staffs)]
pub struct NewStaff {
    pub name: String,
    pub position: String,
    pub department: String,
}
