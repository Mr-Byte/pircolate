use failure::Fail;

#[derive(Debug, Fail)]
pub enum MessageParseError {
    #[fail(display = "Unexpected End of Input (malformed message).")]
    UnexpectedEndOfInput,
}

pub type MessageParseResult<T> = Result<T, MessageParseError>;
