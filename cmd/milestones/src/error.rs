pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Config(#[from] config::error::Error),

    #[error(transparent)]
    Action(#[from] action::error::Error),

    #[error(transparent)]
    Octocrab(#[from] octocrab::Error),

    #[error(transparent)]
    Any(#[from] anyhow::Error),
}
