use crate::error::MessageParseError;
use crate::message::Message;

type Result<T> = ::std::result::Result<T, MessageParseError>;

/// Constructs a message containing a PONG command targeting the specified host.
pub fn pong(host: &str) -> Result<Message> {
    Message::try_from(format!("PONG {}", host))
}

/// Constructs a message containing a PASS command with the specified password.
pub fn pass(pass: &str) -> Result<Message> {
    Message::try_from(format!("PASS {}", pass))
}

/// Constructs a message containing a NICK command with the specified nickname.
pub fn nick(nick: &str) -> Result<Message> {
    Message::try_from(format!("NICK {}", nick))
}

/// Constructs a message containing a USER command with the specified username and real name.
pub fn user(username: &str, real_name: &str) -> Result<Message> {
    Message::try_from(format!("USER {} 0 * :{}", username, real_name))
}

/// Constructs a message containing an IRCv3 CAP REQ command for the specified capability.
pub fn cap_req(cap: &str) -> Result<Message> {
    Message::try_from(format!("CAP REQ :{}", cap))
}

/// Constructs a message containing a JOIN command for the specified channel.
/// The `channels` parameter is a comma separated list of channels to join.
/// The `keys` parameter is an optional comma separated list of passwords for the channels being joined.
pub fn join(channels: &str, keys: Option<&str>) -> Result<Message> {
    let command = if let Some(keys) = keys {
        format!("JOIN {} {}", channels, keys)
    } else {
        format!("JOIN {}", channels)
    };

    Message::try_from(command)
}

/// Constructs a message containing a PRIVMSG command sent to the specified targets with the given message.
pub fn priv_msg(targets: &str, message: &str) -> Result<Message> {
    Message::try_from(format!("PRIVMSG {} :{}", targets, message))
}
