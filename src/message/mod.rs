//! The message module contains the `Message` struct which represents an
//! IRC message either being received from the server or sent by the client.
//!
//! The module also contains several constructor methods for constructing
//! messages to be sent to the server.

mod parser;

#[cfg(feature = "twitch-client")]
mod twitch;
#[cfg(feature = "twitch-client")]
pub use twitch::*;

use crate::arc_slice::ArcSlice;
use crate::command::{ArgumentIter, Command};
use crate::error::MessageParseError;
use crate::tag::{Tag, TagIter};

use bytes::Bytes;

use std::ops::Range;

type MesssageParseResult = Result<Message, MessageParseError>;

#[derive(Clone)]
struct PrefixRange {
    raw_prefix: Range<usize>,
    prefix: Range<usize>,
    user: Option<Range<usize>>,
    host: Option<Range<usize>>,
}

type TagRange = (Range<usize>, Option<Range<usize>>);

/// Representation of IRC messages that splits a message into its constituent
/// parts specified in RFC1459 and the IRCv3 spec.
#[derive(Clone)]
pub struct Message {
    message: Bytes,
    tags: Option<ArcSlice<TagRange>>,
    prefix: Option<PrefixRange>,
    command: Range<usize>,
    arguments: Option<ArcSlice<Range<usize>>>,
}

impl Message {
    /// A strongly typed interface for determining the type of the command
    /// and retrieving the values of the command.
    pub fn command<'a, T>(&'a self) -> Option<T>
    where
        T: Command<'a>,
    {
        <T as Command>::try_match(self.raw_command(), self.raw_args())
    }

    /// A strongly type way of accessing a specified tag associated with
    /// a message.
    pub fn tag<'a, T>(&'a self) -> Option<T>
    where
        T: Tag<'a>,
    {
        <T as Tag>::try_match(self.raw_tags())
    }

    /// Retrieves the prefix for this message, if there is one.  If there is either
    /// a user or host associated with the prefix, it will also return those.
    pub fn prefix(&self) -> Option<(&str, Option<&str>, Option<&str>)> {
        if let Some(ref prefix_range) = self.prefix {
            let user = prefix_range
                .user
                .clone()
                .map(|user| &self.raw_message()[user]);
            let host = prefix_range
                .host
                .clone()
                .map(|host| &self.raw_message()[host]);

            Some((&self.raw_message()[prefix_range.prefix.clone()], user, host))
        } else {
            None
        }
    }

    /// Get an iterator to the raw key/value pairs of tags associated with
    /// this message.
    pub fn raw_tags(&self) -> TagIter {
        if let Some(ref tags) = self.tags {
            TagIter::new(&self.raw_message(), tags.iter())
        } else {
            TagIter::new(&self.raw_message(), [].iter())
        }
    }

    /// Attempt to get the raw prefix value associated with this message.
    pub fn raw_prefix(&self) -> Option<&str> {
        if let Some(ref prefix_range) = self.prefix {
            Some(&self.raw_message()[prefix_range.raw_prefix.clone()])
        } else {
            None
        }
    }

    /// Retrieve the raw command associated with this message.
    pub fn raw_command(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.message[self.command.clone()]) }
    }

    /// Get an iterator to the raw arguments associated with this message.
    pub fn raw_args(&self) -> ArgumentIter {
        if let Some(ref arguments) = self.arguments {
            ArgumentIter::new(self.raw_message(), arguments.iter())
        } else {
            ArgumentIter::new(self.raw_message(), [].iter())
        }
    }

    /// Get the raw IRC command this message was constrcuted from.
    #[inline]
    pub fn raw_message(&self) -> &str {
        // NOTE: This is safe because the only way to obtain a Message is to run it through the parser
        // which validates that the input is a valid UTF-8 string before proceeding.
        unsafe { std::str::from_utf8_unchecked(&self.message) }
    }

    pub fn try_from(
        value: impl std::convert::TryInto<Message, Error = MessageParseError>,
    ) -> MesssageParseResult {
        value.try_into()
    }
}

use std::convert::TryFrom;

impl TryFrom<String> for Message {
    type Error = MessageParseError;

    fn try_from(value: String) -> MesssageParseResult {
        parser::parse_message(value)
    }
}

impl TryFrom<Bytes> for Message {
    type Error = MessageParseError;

    fn try_from(value: Bytes) -> MesssageParseResult {
        parser::parse_message(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for Message {
    type Error = MessageParseError;

    fn try_from(value: &'a [u8]) -> MesssageParseResult {
        parser::parse_message(value.to_vec())
    }
}

impl<'a> TryFrom<&'a str> for Message {
    type Error = MessageParseError;

    fn try_from(value: &'a str) -> MesssageParseResult {
        parser::parse_message(value.as_bytes().to_vec())
    }
}
