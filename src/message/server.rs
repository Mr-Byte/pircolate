use message::Message;
use error::Result;

/// Constructs a message containing a PING command targeting the specified host.
pub fn ping(host: &str) -> Result<Message> {
    Message::try_from(format!("PING :{}", host))
}

/// Constructs a message containing a WELCOME numeric with the specified contents.
pub fn welcome(target: &str, message: &str) -> Result<Message> {
    Message::try_from(format!("001 {} :{}", target, message))
}

/// Constructs a message containing a YOURHOST numeric with the specified contents.
pub fn your_host(target: &str, message: &str) -> Result<Message> {
    Message::try_from(format!("002 {} :{}", target, message))
}

/// Constructs a message containing a CREATED numeric with the specified contents.
pub fn created(target: &str, message: &str) -> Result<Message> {
    Message::try_from(format!("003 {} :{}", target, message))
}

/// Constructs a message containing a MYINFO numeric with the specified contents.
pub fn server_info(target: &str, message: &str) -> Result<Message> {
    Message::try_from(format!("004 {} :{}", target, message))
}
