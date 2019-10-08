#[derive(Debug, Error)]
pub enum MessageParseError {
    #[error(display = "Unexpected End of Input (malformed message).")]
    UnexpectedEndOfInput,
}

pub type MessageParseResult<T> = Result<T, MessageParseError>;
