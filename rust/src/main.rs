use anyhow::Result;
use dotenv::dotenv;
use std::env;
use tracing::{Level, info};

mod db;
mod handlers;
mod models;
mod services;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    // Database URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Redis URL
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

    info!("Starting blog backend service");
    info!("Database: {}", database_url);
    info!("Redis: {}", redis_url);

    // Initialize database pool
    let db_pool = db::create_pool(&database_url).await?;

    // Initialize Redis client
    let redis_client = redis::Client::open(redis_url.as_str())?;

    // Start gRPC server
    let grpc_addr = "0.0.0.0:50051".parse()?;
    info!("gRPC server listening on {}", grpc_addr);

    // Start HTTP server (Axum)
    let http_addr = "0.0.0.0:8080".parse()?;
    info!("HTTP server listening on {}", http_addr);

    // Run both servers concurrently
    tokio::try_join!(
        services::run_grpc_server(grpc_addr, db_pool.clone(), redis_client.clone()),
        handlers::run_http_server(http_addr, db_pool, redis_client),
    )?;

    Ok(())
}
