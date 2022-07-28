use async_graphql::ErrorExtensions;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Could not find resource")]
    NotFound,
    #[error("Generic")]
    Generic(#[from] anyhow::Error),
}

impl ErrorExtensions for ApiError {
    fn extend(&self) -> async_graphql::Error {
        self.extend_with(|err, e| match err {
            ApiError::NotFound | ApiError::Generic(_) => {}
        })
    }
}
