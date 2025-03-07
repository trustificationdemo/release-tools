pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Env(#[from] envy::Error),

    #[error(transparent)]
    Octocrab(#[from] octocrab::Error),
}
