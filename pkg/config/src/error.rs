pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Serde(#[from] serde_yml::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
