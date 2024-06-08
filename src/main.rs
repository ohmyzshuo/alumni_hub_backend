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
use axum::routing::{delete, get, post, put};
use tracing::info;

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
    let arc_pool = Arc::new(pool);

    // Instantiate StaffServiceImpl with Arc<DbPool>
    let staff_service = Arc::new(StaffServiceImpl::new(arc_pool.clone())) as Arc<dyn StaffService + Send + Sync>;

    // Create the application with routes and middlewares
    let app = Router::new()
        .route("/staff", post(handlers::staff_handler::create))
        .route("/staff/:id", axum::routing::get(handlers::staff_handler::read))
        .route("/staff/:id", axum::routing::put(handlers::staff_handler::update))
        .route("/staff/:id", axum::routing::delete(handlers::staff_handler::delete))
        .layer(Extension(staff_service.clone())); // Ensure the staff_service is added as an extension



    // Bind the listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:6666").await.unwrap();
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
