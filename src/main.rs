use axum::{Extension, Router, ServiceExt};
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::services::staff_service::{StaffServiceImpl, StaffService};

mod handlers;
mod models;
mod schema;
mod services;
mod db;
mod routes;
mod utils;
mod errors;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use anyhow::Result;
use axum::http::Method;
use axum::routing::{delete, get, post, put};
use tower_http::cors::{AllowHeaders, AllowOrigin, Any, CorsLayer};
use tracing::info;
use crate::handlers::alumni_handler::{get_alumni};
use crate::services::alumni_service::AlumniService;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load environment variables
    info!("Loading environment variables...");
    dotenv::dotenv().ok();

    // Setup database connection pool
    info!("Setting up database connection pool...");
    let database_url = match std::env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => {
            eprintln!("DATABASE_URL must be set");
            return Err(anyhow::anyhow!("DATABASE_URL must be set"));
        }
    };

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = match Pool::builder().build(manager) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Failed to create pool: {:?}", e);
            return Err(anyhow::anyhow!("Failed to create pool"));
        }
    };
    // Wrap the pool in Arc
    let arc_pool = Arc::new(pool.clone());

    // Instantiate StaffServiceImpl with Arc<DbPool>
    let staff_service = Arc::new(StaffServiceImpl::new(arc_pool.clone())) as Arc<dyn StaffService + Send + Sync>;
    let alumni_service = Arc::new(AlumniService::new(Arc::new(pool.clone())));

    // Create the application with routes and middlewares
    let app = Router::new()
        .route("/staff", post(handlers::staff_handler::create))
        .route("/staff/:id", axum::routing::get(handlers::staff_handler::read))
        .route("/staff/:id", axum::routing::put(handlers::staff_handler::update))
        .route("/staff/:id", axum::routing::delete(handlers::staff_handler::delete))
        .route("/staffs", get(handlers::staff_handler::list))
        .layer(Extension(staff_service.clone()))
        .route("/alumni", get(get_alumni))
        .layer(Extension(alumni_service.clone()))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact("http://localhost:63343".parse().unwrap()))
                .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers(AllowHeaders::list(vec![
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                    axum::http::header::ACCEPT,
                ]))
                .allow_credentials(true),
        );


    // Bind the listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    // Run the server
    match axum::serve(listener, app.into_make_service()).await {
        Ok(_) => tracing::info!("Server is running"),
        Err(e) => {
            tracing::error!("Server error: {:?}", e);
            return Err(e.into());
        }
    }
    Ok(())
}
