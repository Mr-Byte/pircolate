//! The command module contains everything needed to perform strongly typed access
//! to commands associated with a message.

#[cfg(feature = "twitch-client")]
mod twitch;
#[cfg(feature = "twitch-client")]
pub use twitch::*;

use std::ops::Range;
use std::slice::Iter;

/// An implementation of Iterator that iterates over the arguments of a `Message`.
#[derive(Clone)]
pub struct ArgumentIter<'a> {
    source: &'a str,
    iter: Iter<'a, Range<usize>>,
}

impl<'a> ArgumentIter<'a> {
    pub(crate) fn new(source: &'a str, iter: Iter<'a, Range<usize>>) -> ArgumentIter<'a> {
        ArgumentIter { source, iter }
    }
}

impl<'a> Iterator for ArgumentIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|range| &self.source[range.clone()])
    }
}

impl<'a> DoubleEndedIterator for ArgumentIter<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.iter
            .next_back()
            .map(|range| &self.source[range.clone()])
    }
}

/// The `Command` trait is a trait that's implemented by types wishing to provide command
/// parsing capability for usage with the `Message::command` method.
pub trait Command<'a> {
    /// Provides the name of the command to be matched. Examples include `PRIVMSG` or `PING`.
    const NAME: &'static str;

    /// This method takes in an iterator of arguments associated with a `Message` and attempts
    /// to parse the arguments into a matched `Command`.  If no match is found, None is returned.
    fn parse(arguments: ArgumentIter<'a>) -> Option<Self>
    where
        Self: Sized;

    /// A default implementation that takes in the given command name and arguments and attempts to match
    /// the command and parse the arguments into a strongly typed representation. If there is no match
    /// or the parse fails, it returns `None`.
    fn try_match(command: &str, arguments: ArgumentIter<'a>) -> Option<Self>
    where
        Self: Sized,
    {
        if command == Self::NAME {
            Self::parse(arguments)
        } else {
            None
        }
    }
}

/// A macro for simplifying the process of matching commands.
///
/// # Examples
///
/// Match all PING commands.
///
/// ```
/// # #[macro_use] extern crate pircolate;
/// #
/// # use pircolate::message;
/// # use pircolate::command::Ping;
/// # use std::convert::TryFrom;
/// #
/// # fn main() {
/// #   let msg = message::Message::try_from("TEST bob :hello, world!").unwrap();
/// command_match! {
///     msg => {
///         Ping(source) => println!("{}", source),
///         _ => ()
///     }
/// };
/// # }
/// ```
#[macro_export]
macro_rules! command_match {
    (@message=$message:expr => $command:pat => $body:expr) => {{
        let $command = $message;
        $body
    }};

    (@message=$message:expr => $command:pat => $body:expr, $($rest:tt)*) => {
        match $message.command() {
            Some($command) => $body,
            _ => command_match!(@message=$message => $($rest)*)
        }
    };

    ($message:expr => { $($rest:tt)* }) => {{
        let message = $message;
        command_match!(@message=message => $($rest)*)
    }};
}

/// A macro for creating implementations of basic commands with up to four
/// &str arguments.
///
/// # Examples
///
/// Simple command "TEST" with two &str arguments.
///
/// ```
/// # #[macro_use] extern crate pircolate;
/// #
/// # use pircolate::message;
/// # use pircolate::command::Ping;
/// # use pircolate::command::ArgumentIter;
/// # use std::convert::TryFrom;
/// #
/// command! {
///   /// Some command!
///   ("TEST" => Test(user, message))
/// }
/// #
/// # fn main() {
/// #   let msg = message::Message::try_from("TEST bob :hello, world!").unwrap();
/// if let Some(Test(user, message)) = msg.command::<Test>() {
///     println!("<{}> {}", user, message);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! command {
    ($(#[$meta:meta])* ($command:expr => $command_name:ident())) => {
        $(#[$meta])*
        pub struct $command_name;

        impl<'a> $crate::command::Command<'a> for $command_name {
            const NAME: &'static str = $command;

            fn parse(_: ArgumentIter<'a>) -> Option<$command_name> {
                Some($command_name)
            }
        }
    };

    ($(#[$meta:meta])* ($command:expr => $command_name:ident($($name:ident),+))) => {
        $(#[$meta])*

        pub struct $command_name<'a>($(pub expand_param!($name)),+);

        impl<'a> $crate::command::Command<'a> for $command_name<'a> {
            const NAME: &'static str = $command;

            fn parse(mut arguments: ArgumentIter<'a>) -> Option<$command_name<'a>> {
                $(let $name = arguments.next()?;)+
                Some($command_name($($name),*))
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! expand_param {
    ($i:ident) => { &'a str };
}
