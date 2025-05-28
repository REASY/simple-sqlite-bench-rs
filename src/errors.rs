use thiserror::Error;

#[derive(Error, Debug)]
#[error(transparent)]
pub struct AppError(Box<ErrorKind>);

#[derive(Error, Debug)]
#[error(transparent)]
pub enum ErrorKind {
    #[error("Io: {0}")]
    Io(#[from] std::io::Error),
    #[error("Sqlx: {0}")]
    Sqlx(#[from] sqlx::Error),
}

impl<E> From<E> for AppError
where
    ErrorKind: From<E>,
{
    fn from(err: E) -> Self {
        AppError(Box::new(ErrorKind::from(err)))
    }
}
pub type AppResult<T> = std::result::Result<T, AppError>;
