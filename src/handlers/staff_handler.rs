use crate::services::staff::{create_staff, get_staff };
use crate::models::staff::{ NewStaff, Staff };
use axum::{ extract::{ Path, Extension, Json }, http::StatusCode, response::IntoResponse };
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use std::sync::Arc;
use std::future::Future;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn create(
    Json(new_staff): Json<NewStaff>,
    Extension(pool): Extension<Arc<DbPool>>
) -> impl IntoResponse {
    let conn = pool.get().expect("Failed to get DB connection");
    match create_staff(&conn, new_staff) {
        Ok(staff) => (StatusCode::CREATED, Json(staff)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create staff").into_response(),
    }
}

pub async fn read(
    Path(staff_id): Path<i32>,
    Extension(pool): Extension<Arc<DbPool>>
) -> impl IntoResponse {
    let conn = pool.get().expect("Failed to get DB connection");
    match get_staff(&conn, staff_id) {
        Ok(staff) => (StatusCode::OK, Json(staff)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Staff not found").into_response(),
    }
}
