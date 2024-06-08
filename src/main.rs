use axum::{Extension};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;
use std::sync::Arc;
use std::net::SocketAddr;

mod api;
mod handlers;
mod models;
mod schema;
mod services;
mod db;
mod routes;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables
    dotenv::dotenv().ok();

    // Setup database connection pool
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Create the application with routes and middlewares
    let app = routes::create_routes()
        .layer(Extension(Arc::new(pool)));

    // Set up the listener address
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    // Run the server
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
