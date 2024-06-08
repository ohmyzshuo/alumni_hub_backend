use crate::models::{alumni::Alumni, alumni::NewAlumni, alumni::UpdateAlumni};
use crate::schema::alumnis::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;
use serde_json::Value;
use std::error::Error;
use crate::utils::auth_utils::{extract_login_name, hash_password};

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct AlumniService {
    pool: Arc<DbPool>,
}

impl AlumniService {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }

    pub async fn get_alumni(
        &self,
        page: i64,
        page_size: i64,
        search_query: &str,
    ) -> Result<(Vec<Alumni>, i64), Box<dyn Error>> {
        let mut conn = self.pool.get()?;
        let offset = (page - 1) * page_size;

        let alumni_list = alumnis
            .filter(is_hidden.eq(false))
            .filter(name.ilike(format!("%{}%", search_query)))
            .or_filter(email.ilike(format!("%{}%", search_query)))
            .or_filter(phone.ilike(format!("%{}%", search_query)))
            .or_filter(matric_no.ilike(format!("%{}%", search_query)))
            .offset(offset)
            .limit(page_size)
            .load::<Alumni>(&mut conn)?;

        let total: i64 = alumnis
            .filter(is_hidden.eq(false))
            .filter(name.ilike(format!("%{}%", search_query)))
            .or_filter(email.ilike(format!("%{}%", search_query)))
            .or_filter(phone.ilike(format!("%{}%", search_query)))
            .or_filter(matric_no.ilike(format!("%{}%", search_query)))
            .count()
            .get_result(&mut conn)?;

        Ok((alumni_list, total))
    }

    // pub async fn create_alumni(&self, new_alumni: NewAlumni) -> Result<Alumni, Box<dyn Error>> {
    //     let mut conn = self.pool.get()?;
    //
    //     if let Some(_) = alumnis.filter(matric_no.eq(&new_alumni.matric_no)).first::<Alumni>(&mut conn).optional()? {
    //         return Err("duplicated matric_no".into());
    //     }
    //
    //     let login_name = extract_login_name(&new_alumni.matric_no)?;
    //     let hashed_password = hash_password(&new_alumni.password.unwrap_or(login_name))?;
    //     let new_alumni = NewAlumni { password: Some(hashed_password), ..new_alumni };
    //
    //     diesel::insert_into(alumnis)
    //         .values(&new_alumni)
    //         .get_result(&mut conn)
    //         .map_err(|e| e.into())
    // }
    //
    // pub async fn update_alumni(&self, id: i32, updated_alumni: UpdateAlumni) -> Result<Alumni, Box<dyn Error>> {
    //     let conn = self.pool.get()?;
    //
    //     if let Some(existing_alumni) = alumnis.filter(matric_no.eq(&updated_alumni.matric_no)).first::<Alumni>(&conn).optional()? {
    //         if existing_alumni.id != id {
    //             return Err("duplicated matric_no".into());
    //         }
    //     }
    //
    //     diesel::update(alumnis.find(id))
    //         .set(&updated_alumni)
    //         .get_result(&conn)
    //         .map_err(|e| e.into())
    // }
    //
    // pub async fn delete_alumni(&self, id: i32) -> Result<(), Box<dyn Error>> {
    //     let conn = self.pool.get()?;
    //
    //     diesel::update(alumnis.find(id))
    //         .set(is_hidden.eq(true))
    //         .execute(&conn)
    //         .map(|_| ())
    //         .map_err(|e| e.into())
    // }
}
