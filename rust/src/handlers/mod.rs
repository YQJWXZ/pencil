use anyhow::Result;
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;

use crate::db::Pool;

pub async fn run_http_server(
    addr: SocketAddr,
    _db_pool: Pool,
    _redis_client: redis::Client,
) -> Result<()> {
    let app = Router::new().route("/health", get(health_check));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
