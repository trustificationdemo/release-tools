pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("Invalid prefix (title {title:?}, emoji {emoji:?})")]
    InvalidTitle {
        title: String,
        emoji: Option<String>,
    },
}
