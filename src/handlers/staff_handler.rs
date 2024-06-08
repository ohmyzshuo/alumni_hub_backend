use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    debug_handler,
};
use std::sync::Arc;
use tracing::debug;
use crate::services::staff_service::StaffService;
use crate::models::staff::{NewStaff, UpdateStaff};

#[debug_handler]
pub async fn create(
    Extension(staff_service): Extension<Arc<dyn StaffService + Send + Sync>>,
    Json(payload): Json<NewStaff>,
) -> impl IntoResponse {
    match Extension(staff_service).create_staff(payload).await {
        Ok(staff) => (StatusCode::CREATED, Json(staff)).into_response(),
        Err(err) => {
            eprintln!("Failed to create staff: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

#[debug_handler]
pub async fn read(
    Extension(staff_service): Extension<Arc<dyn StaffService + Send + Sync>>,
    Path(staff_id): Path<i32>,
) -> impl IntoResponse {
    match Extension(staff_service.clone()).get_staff(staff_id).await {
        Ok(staff) => debug!("ssss1"),
        Err(_) => debug!("ssss2"),
    }

    debug!("ssss3");

    if let Ok(staff) = staff_service.get_staff(staff_id).await {
        (StatusCode::OK, Json(staff)).into_response()
    } else {
        (StatusCode::NOT_FOUND, "Staff not found11111").into_response()
    }
}

#[debug_handler]
pub async fn update(
    Extension(staff_service): Extension<Arc<dyn StaffService + Send + Sync>>,
    Path(staff_id): Path<i32>,
    Json(updated_staff): Json<UpdateStaff>,
) -> impl IntoResponse {
    match Extension(staff_service).update_staff(staff_id, updated_staff).await {
        Ok(staff) => (StatusCode::OK, Json(staff)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update staff").into_response(),
    }
}

#[debug_handler]
pub async fn delete(
    Extension(staff_service): Extension<Arc<dyn StaffService + Send + Sync>>,
    Path(staff_id): Path<i32>,
) -> impl IntoResponse {
    match Extension(staff_service).delete_staff(staff_id).await {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete staff").into_response(),
    }
}
