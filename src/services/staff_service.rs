use crate::models::staff::{NewStaff, Staff, UpdateStaff};
use crate::schema::staffs::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::sync::Arc;
use async_trait::async_trait;
use crate::errors::ServiceError;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[async_trait]
pub trait StaffService: Send + Sync {
    async fn create_staff(&self, new_staff: NewStaff) -> Result<Staff, ServiceError>;
    async fn get_staff(&self, staff_id: i32) -> Result<Staff, ServiceError>;
    async fn get_staffs(&self, page: i32, page_size: i32) -> Result<(Vec<Staff>, i64), ServiceError>;
    async fn update_staff(&self, staff_id: i32, updated_staff: UpdateStaff) -> Result<Staff, ServiceError>;
    async fn delete_staff(&self, staff_id: i32) -> Result<(), ServiceError>;
}

pub struct StaffServiceImpl {
    pool: Arc<DbPool>,
}

impl StaffServiceImpl {
    pub fn new(pool: Arc<DbPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StaffService for StaffServiceImpl {
    async fn create_staff(&self, new_staff: NewStaff) -> Result<Staff, ServiceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(ServiceError::from)?;
            diesel::insert_into(staffs).values(&new_staff).get_result(&mut conn).map_err(ServiceError::from)
        })
            .await?
    }

    async fn get_staff(&self, staff_id: i32) -> Result<Staff, ServiceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(ServiceError::from)?;
            staffs.find(staff_id).get_result(&mut conn).map_err(ServiceError::from)
        })
            .await?
    }

    async fn get_staffs(&self, page: i32, page_size: i32) -> Result<(Vec<Staff>, i64), ServiceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(ServiceError::from)?;
            let offset = (page - 1) * page_size;
            let total: i64 = staffs.count().get_result(&mut conn).map_err(ServiceError::from)?;
            let results = staffs
                .limit(page_size.into())
                .offset(offset.into())
                .load::<Staff>(&mut conn)
                .map_err(|e| match e {
                    diesel::result::Error::DeserializationError(err) => {
                        if let Some(inner) = err.source() {
                            if inner.to_string().contains("Unrecognized enum variant") {
                                ServiceError::UnrecognizedEnumVariant
                            } else {
                                ServiceError::DieselError(diesel::result::Error::DeserializationError(err))
                            }
                        } else {
                            ServiceError::DieselError(diesel::result::Error::DeserializationError(err))
                        }
                    },
                    _ => ServiceError::from(e),
                })?;
            Ok((results, total))
        })
            .await?
    }

    async fn update_staff(&self, staff_id: i32, updated_staff: UpdateStaff) -> Result<Staff, ServiceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(ServiceError::from)?;
            diesel::update(staffs.find(staff_id)).set(&updated_staff).get_result(&mut conn).map_err(ServiceError::from)
        })
            .await?
    }

    async fn delete_staff(&self, staff_id: i32) -> Result<(), ServiceError> {
        let pool = self.pool.clone();
        tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(ServiceError::from)?;
            diesel::delete(staffs.find(staff_id)).execute(&mut conn).map_err(ServiceError::from)
        })
            .await??;
        Ok(())
    }
}
