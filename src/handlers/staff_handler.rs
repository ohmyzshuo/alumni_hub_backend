use axum::{
    extract::{Path, Extension, Json},
    http::StatusCode,
    response::IntoResponse,
    debug_handler,
};
use std::sync::Arc;
use axum::extract::Query;
use serde::Deserialize;
use serde_json::json;
use tracing::debug;
use crate::services::staff_service::StaffService;
use crate::models::staff::{NewStaff, UpdateStaff};
#[derive(Deserialize)]
pub struct Pagination {
    page: i32,
    page_size: i32,
}
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
#[debug_handler]
pub async fn list(
    Extension(staff_service): Extension<Arc<dyn StaffService + Send + Sync>>,
    Query(pagination): Query<Pagination>,
) -> impl IntoResponse {
    debug!("tong le");
    let page = pagination.page;
    let page_size = pagination.page_size;
    match staff_service.get_staffs(page, page_size).await {
        Ok((staffs, total)) => {
            let total_pages = (total as f64 / page_size as f64).ceil() as i32;
            let response = serde_json::json!({
                "code": 200,
                "data": staffs,
                "message": "Success",
                "meta": {
                    "page": page,
                    "page_size": page_size,
                    "total": total,
                    "total_pages": total_pages
                }
            });
            (StatusCode::OK, Json(response)).into_response()
        }
        Err(err) => {
            eprintln!("Failed to list staffs: {:?}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}