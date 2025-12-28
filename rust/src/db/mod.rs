use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[allow(dead_code)]
pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> anyhow::Result<Pool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().max_size(20).build(manager)?;
    Ok(pool)
}
