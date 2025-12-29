use anyhow::Result;
use std::net::SocketAddr;
#[allow(unused_imports)]
use tonic::transport::Server;

mod article;
mod asset;
mod user;

#[allow(unused_imports)]
pub use article::ArticleServiceImpl;
#[allow(unused_imports)]
pub use asset::AssetServiceImpl;
#[allow(unused_imports)]
pub use user::UserServiceImpl;

use crate::db::Pool;

pub async fn run_grpc_server(
    _addr: SocketAddr,
    _db_pool: Pool,
    _redis_client: redis::Client,
) -> Result<()> {
    // gRPC服务器实现将在这里
    // 目前返回占位符
    Ok(())
}
