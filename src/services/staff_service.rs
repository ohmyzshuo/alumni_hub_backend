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
        let result = tokio::task::spawn_blocking(move || {
            let mut conn = pool.get().map_err(|e| {
                eprintln!("Failed to get connection from pool: {:?}", e);
                ServiceError::from(e)
            })?;
            let staff = staffs.find(staff_id).get_result(&mut conn).map_err(|e| {
                eprintln!("Failed to find staff with id {}: {:?}", staff_id, e);
                ServiceError::from(e)
            })?;
            Ok(staff)
        })
            .await;

        result.map_err(|e| {
            eprintln!("Error in spawn_blocking: {:?}", e);
            ServiceError::from(e)
        })?
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
