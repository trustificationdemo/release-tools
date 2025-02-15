pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Action(#[from] action::error::Error),

    #[error(transparent)]
    PR(#[from] pr::error::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Unable to unmarshal PullRequest {file_path:?}")]
    UnmarshalPullRequest {
        file_path: String,
        err: serde_json::Error,
    },
}
