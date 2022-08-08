use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("local system io error")]
    IO(#[from] std::io::Error),
    #[error("sqlite error")]
    SQL(#[from] rusqlite::Error),

    #[error("time ranges logic error")]
    RangesUpdateError,
}

pub type Result<T> = std::result::Result<T, Error>;
