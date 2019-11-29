use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageParseError {
    #[error("Unexpected End of Input (malformed message).")]
    UnexpectedEndOfInput,
}

pub type MessageParseResult<T> = Result<T, MessageParseError>;
