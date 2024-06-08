use crate::services::alumni_service::AlumniService;
use crate::models::alumni::{NewAlumni, UpdateAlumni};
use axum::{
    extract::{Extension, Json, Path, Query},
    response::IntoResponse,
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::json;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<i64>,
    page_size: Option<i64>,
    search: Option<String>,
}

pub async fn get_alumni(
    Query(pagination): Query<Pagination>,
    Extension(service): Extension<Arc<AlumniService>>,
) -> impl IntoResponse {
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(15);
    let search_query = pagination.search.unwrap_or_default();

    match service.get_alumni(page, page_size, &search_query).await {
        Ok((alumni, total)) => {
            let total_pages = (total + page_size - 1) / page_size;
            (StatusCode::OK, Json(json!({
                "code": 200,
                "message": "Success",
                "data": alumni,
                "meta": {
                    "page": page,
                    "page_size": page_size,
                    "total": total,
                    "total_pages": total_pages,
                }
            })))
        }
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
            "code": 500,
            "message": "Error fetching alumni",
        }))),
    }
}

// pub async fn create_alumni(
//     Json(new_alumni): Json<NewAlumni>,
//     Extension(service): Extension<Arc<AlumniService>>,
// ) -> impl IntoResponse {
//     match service.create_alumni(new_alumni).await {
//         Ok(alumni) => (StatusCode::CREATED, Json(json!({
//             "code": 201,
//             "message": "Alumni created successfully",
//             "data": alumni,
//         }))),
//         Err(e) => {
//             if e.to_string() == "duplicated matric_no" {
//                 (StatusCode::BAD_REQUEST, Json(json!({
//                     "code": 230,
//                     "message": "Duplicated matric_no",
//                 })))
//             } else {
//                 (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
//                     "code": 500,
//                     "message": e.to_string(),
//                 })))
//             }
//         }
//     }
// }
//
// pub async fn update_alumni(
//     Path(id): Path<i32>,
//     Json(updated_alumni): Json<UpdateAlumni>,
//     Extension(service): Extension<Arc<AlumniService>>,
// ) -> impl IntoResponse {
//     match service.update_alumni(id, updated_alumni).await {
//         Ok(alumni) => (StatusCode::OK, Json(json!({
//             "code": 200,
//             "message": "Alumni updated successfully",
//             "data": alumni,
//         }))),
//         Err(e) => {
//             if e.to_string() == "duplicated matric_no" {
//                 (StatusCode::BAD_REQUEST, Json(json!({
//                     "code": 230,
//                     "message": "Duplicated matric_no",
//                 })))
//             } else {
//                 (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
//                     "code": 500,
//                     "message": e.to_string(),
//                 })))
//             }
//         }
//     }
// }
//
// pub async fn delete_alumni(
//     Path(id): Path<i32>,
//     Extension(service): Extension<Arc<AlumniService>>,
// ) -> impl IntoResponse {
//     match service.delete_alumni(id).await {
//         Ok(_) => (StatusCode::OK, Json(json!({
//             "code": 200,
//             "message": "Alumni hidden successfully",
//         }))),
//         Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({
//             "code": 500,
//             "message": "Error hiding alumni",
//         }))),
//     }
// }
