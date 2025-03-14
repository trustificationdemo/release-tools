pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error(
        "Invalid prefix (title: {title}, emoji: {emoji:#?}).\nValid prefixes are:\n:sparkles:\n:bug:\n:book:\n:seedling:\n:warning:\n:ghost:\n"
    )]
    InvalidTitle {
        title: String,
        emoji: Option<String>,
    },
}
