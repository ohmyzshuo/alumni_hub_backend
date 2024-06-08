use thiserror::Error;
use diesel::result::Error as DieselError;
use r2d2::Error as R2D2Error;
use tokio::task::JoinError;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    DieselError(#[from] DieselError),
    #[error("Connection pool error: {0}")]
    R2D2Error(#[from] R2D2Error),
    #[error("Join error: {0}")]
    JoinError(#[from] JoinError),
}
