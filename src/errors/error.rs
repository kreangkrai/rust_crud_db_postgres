
use deadpool_postgres::PoolError;
use derive_more::{Display,From};
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;
#[derive(Display,From,Debug)]
pub enum MyError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}
impl std::error::Error for MyError{}