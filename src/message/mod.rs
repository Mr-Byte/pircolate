//! The message module contains the `Message` struct which represents an
//! IRC message either being received from the server or sent by the client.
//!
//! The module also contains several constructor methods for constructing
//! messages to be sent to the server.

mod parser;

use error::Result;
use command::{Command, ArgumentIter};
use tag::{Tag, TagIter};

use std::ops::Range;

#[derive(Clone, PartialEq, Eq, Debug)]
struct PrefixRange {
    raw_prefix: Range<usize>,
    prefix: Range<usize>,
    user: Option<Range<usize>>, 
    host: Option<Range<usize>>
}

type TagRange = (Range<usize>, Option<Range<usize>>);

/// Representation of IRC messages that splits a message into its constituent
/// parts specified in RFC1459 and the IRCv3 spec.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Message {
    message: String,
    tags: Option<Vec<TagRange>>,
    prefix: Option<PrefixRange>,
    command: Range<usize>,
    arguments: Option<Vec<Range<usize>>>,
}

impl Message {
    /// Attempt to construct a new message from the given raw IRC message.
    pub fn try_from(value: String) -> Result<Message> {
        let result = parser::parse_message(value)?;

        Ok(result)
    }

    /// A strongly typed interface for determining the type of the command
    /// and retrieving the values of the command.
    pub fn command<'a, T>(&'a self) -> Option<T> where T : Command<'a> {
        <T as Command>::try_match(self.raw_command(), self.raw_args())
    }

    /// A strongly type way of accessing a specified tag associated with
    /// a message.
    pub fn tag<'a, T>(&'a self) -> Option<T> where T : Tag<'a> {
        <T as Tag>::try_match(self.raw_tags())
    }

    /// Retrieves the prefix for this message, if there is one.  If there is either
    /// a user or host associated with the prefix, it will also return those.
    pub fn prefix(&self) -> Option<(&str, Option<&str>, Option<&str>)> {
        if let Some(ref prefix_range) = self.prefix {
            let user = prefix_range.user.clone().map(|user| &self.message[user]);
            let host = prefix_range.host.clone().map(|host| &self.message[host]);

            Some((&self.message[prefix_range.prefix.clone()], user, host))
        } else {
            None
        }
    }

    /// Get an iterator to the raw key/value pairs of tags associated with
    /// this message.
    pub fn raw_tags(&self) -> TagIter {
        if let Some(ref tags) = self.tags {
            TagIter::new(&self.message, tags.iter())
        } else {
            TagIter::new(&self.message, [].iter())
        }
    }

    /// Attempt to get the raw prefix value associated with this message.
    pub fn raw_prefix(&self) -> Option<&str> {
        if let Some(ref prefix_range) = self.prefix {
            Some(&self.message[prefix_range.raw_prefix.clone()])
        } else {
            None
        }
    }

    /// Retrieve the raw command associated with this message.
    pub fn raw_command(&self) -> &str {
        &self.message[self.command.clone()]
    }

    /// Get an iterator to the raw arguments associated with this message.
    pub fn raw_args(&self) -> ArgumentIter {
        if let Some(ref arguments) = self.arguments {
            ArgumentIter::new(&self.message, arguments.iter())
        } else {
            ArgumentIter::new(&self.message, [].iter())
        }
    }

    /// Get the raw IRC command this message was constrcuted from.
    pub fn raw_message(&self) -> &str {
        &self.message
    }
}

/// Constructs a message containing a PING command targeting the specified host.
pub fn ping(host: &str) -> Result<Message> {
    Message::try_from(format!("PING :{}", host))
}

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
pub fn join(channel: &str) -> Result<Message> {
    Message::try_from(format!("JOIN {}", channel))
}

/// Constructs a message containing a PRIVMSG command sent to the specified targets with the given message.
pub fn privmsg(targets: &str, message: &str) -> Result<Message> {
    Message::try_from(format!("PRIVMSG {} :{}", targets, message))
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
