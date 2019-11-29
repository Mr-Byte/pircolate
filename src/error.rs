use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageParseError {
    #[error("Invalid UTF-8 input")]
    InvalidEncoding {
        #[from]
        source: std::str::Utf8Error,
    },
    #[error("Unexpected End of Input (malformed message).")]
    UnexpectedEndOfInput,
}

pub type MessageParseResult<T> = Result<T, MessageParseError>;
