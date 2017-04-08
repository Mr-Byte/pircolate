//! The command module contains everything needed to perform strongly typed access
//! to commands associated with a message.

use std::ops::Range;
use std::slice::Iter;

/// An implementation of Iterator that iterates over the arguments of a `Message`.
#[derive(Clone)]
pub struct ArgumentIter<'a> {
    source: &'a str,
    iter: Iter<'a, Range<usize>>,
}

impl<'a> ArgumentIter<'a> {
    // This is intended for internal usage and thus hidden.
    #[doc(hidden)]
    pub fn new(source: &'a str, iter: Iter<'a, Range<usize>>) -> ArgumentIter<'a> {
        ArgumentIter {
            source: source,
            iter: iter,
        }
    }

    // A helper function that helps map a single argument to another type. If there are no arguments, it returns None.
    #[inline]
    pub fn take_map1<B, F>(mut self, mut f: F) -> Option<B>
        where F: FnMut(&'a str) -> B
    {
        self.next().map(|first| f(first))
    }

    // A helper function that helps map the first two arguments to another type. If there are not enough arguments, it returns None.
    #[inline]
    pub fn take_map2<B, F>(mut self, mut f: F) -> Option<B>
        where F: FnMut(&'a str, &'a str) -> B
    {
        self.next()
            .and_then(|first| self.next().map(|second| f(first, second)))
    }

    // A helper function that helps map the first three arguments to another type. If there are not enough arguments, it returns None.
    #[inline]
    pub fn take_map3<B, F>(mut self, mut f: F) -> Option<B>
        where F: FnMut(&'a str, &'a str, &'a str) -> B
    {
        self.next()
            .and_then(|first| {
                self.next()
                    .and_then(|second| self.next().map(|third| f(first, second, third)))
            })
    }

    // A helper function that helps map the first four arguments to another type. If there are not enough arguments, it returns None.
    #[inline]
    pub fn take_map4<B, F>(mut self, mut f: F) -> Option<B>
        where F: FnMut(&'a str, &'a str, &'a str, &'a str) -> B
    {
        self.next()
            .and_then(|first| {
                self.next()
                    .and_then(|second| {
                        self.next()
                            .and_then(|third| {
                                self.next().map(|fourth| f(first, second, third, fourth))
                            })
                    })
            })
    }
}

impl<'a> Iterator for ArgumentIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|range| &self.source[range.clone()])
    }
}

/// The `Command` trait is a trait that's implemented by types wishing to provide command
/// parsing capability for usage with the `Message::command` method.
pub trait Command<'a> {
    /// Provides the name of the command to be matched. Examples include `PRIVMSG` or `PING`.
    fn name() -> &'static str;

    /// This method takes in an iterator of arguments associated with a `Message` and attempts
    /// to parse the arguments into a matched `Command`.  If no match is found, None is returned.
    fn parse(arguments: ArgumentIter<'a>) -> Option<Self> where Self: Sized;

    /// A default implementation that takes in the given command name and arguments and attempts to match
    /// the command and parse the arguments into a strongly typed representation. If there is no match
    /// or the parse fails, it returns `None`.
    fn try_match(command: &str, arguments: ArgumentIter<'a>) -> Option<Self>
        where Self: Sized
    {
        if command == Self::name() {
            Self::parse(arguments)
        } else {
            None
        }
    }
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
/// command! {
///   /// Some command!
///   ("TEST" => Test(2)) 
/// }
/// # fn main() {}
/// ```
#[macro_export]
macro_rules! command {
    ($(#[$meta:meta])* ($command:expr => $command_name:ident(0))) => {
        $(#[$meta])*
        pub struct $command_name;

        impl<'a> $crate::command::Command<'a> for $command_name {
            fn name() -> &'static str {
                $command
            }

            fn parse(_: $crate::command::ArgumentIter<'a>) -> Option<$command_name> {
                Some($command_name)
            }
        }
    };

    ($(#[$meta:meta])* ($command:expr => $command_name:ident(1))) => {
        $(#[$meta])*
        pub struct $command_name<'a>(pub &'a str);

        impl<'a> $crate::command::Command<'a> for $command_name<'a> {
            fn name() -> &'static str {
                $command
            }

            fn parse(arguments: $crate::command::ArgumentIter<'a>) -> Option<$command_name<'a>> {
                arguments.take_map1(|first| $command_name(first))
            }
        }
    };

    ($(#[$meta:meta])* ($command:expr => $command_name:ident(2))) => {
        $(#[$meta])*
        pub struct $command_name<'a>(pub &'a str, pub &'a str);

        impl<'a> $crate::command::Command<'a> for $command_name<'a> {
            fn name() -> &'static str {
                $command
            }

            fn parse(arguments: $crate::command::ArgumentIter<'a>) -> Option<$command_name<'a>> {
                arguments.take_map2(|first, second| $command_name(first, second))
            }
        }
    };

    ($(#[$meta:meta])* ($command:expr => $command_name:ident(3))) => {
        $(#[$meta])*
        pub struct $command_name<'a>(pub &'a str, pub &'a str, pub &'a str);

        impl<'a> $crate::command::Command<'a> for $command_name<'a> {
            fn name() -> &'static str {
                $command
            }

            fn parse(arguments: $crate::command::ArgumentIter<'a>) -> Option<$command_name<'a>> {
                arguments.take_map3(|first, second, third| $command_name(first, second, third))
            }
        }
    };

    ($(#[$meta:meta])* ($command:expr => $command_name:ident(4))) => {
        $(#[$meta])*
        pub struct $command_name<'a>(pub &'a str, pub &'a str, pub &'a str);

        impl<'a> $crate::command::Command<'a> for $command_name<'a> {
            fn name() -> &'static str {
                $command
            }

            fn parse(arguments: $crate::command::ArgumentIter<'a>) -> Option<$command_name<'a>> {
                arguments.take_map4(|first, second, third, fourth| $command_name(first, second, third, fourth))
            }
        }
    };
}

command! { 
    /// Represents a PING command.  The first element is the host.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate pircolate;
    /// # use pircolate::message;
    /// # use pircolate::command::Ping;
    /// #
    /// # fn main() {
    /// # let msg = message::ping("test.host.com").unwrap();
    ///     if let Some(Ping(host)) = msg.command::<Ping>() {
    ///         println!("PING from {}.", host);
    ///     }
    /// # }
    /// ```
    ("PING" => Ping(1)) 
}

command! {
    /// Represents a PONG command. The first element is the host.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate pircolate;
    /// # use pircolate::message;
    /// # use pircolate::command::Pong;
    /// #
    /// # fn main() {
    /// # let msg = message::pong("test.host.com").unwrap();
    ///     if let Some(Pong(host)) = msg.command::<Pong>() {
    ///         println!("PONG from {}.", host);
    ///     }
    /// # }
    /// ```
    ("PONG" => Pong(1))
}

command! {
    /// Represents a PRIVMSG command.  The first element is the target of the message and
    /// the second eleement is the message.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate pircolate;
    /// # use pircolate::message;
    /// # use pircolate::command::PrivMsg;
    /// #
    /// # fn main() {
    /// # let msg = message::priv_msg("memelord", "memes are great").unwrap();
    ///     if let Some(PrivMsg(user, message)) = msg.command::<PrivMsg>() {
    ///         println!("<{}> {}.", user, message);
    ///     }
    /// # }
    /// ```
    ("PRIVMSG" => PrivMsg(2))
}

command! { 
    /// Represents a WELCOME numeric. The first element is the unsername and the second element is the welcome message.
    ("001" => Welcome(2))
}

command! {
    /// Represents a YOURHOST numeric. The first element is the unsername and the second element is the yourhost message.
    ("002" => YourHost(2))
}

command!{
  /// Represents a CREATED numeric. The first element is the unsername and the second element is the created message.
  ("003" => Created(2))
}

command!{
  /// Represents a MYINFO numeric. The first element is the username and the second element is the server info message.
  ("004" => ServerInfo(2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use message::*;

    #[test]
    fn test_ping_command() {
        let message = ping("test.host.com").unwrap();
        let Ping(host) = message.command::<Ping>().unwrap();

        assert_eq!("test.host.com", host);
    }

    #[test]
    fn test_pong_command() {
        let message = pong("test.host.com").unwrap();
        let Pong(host) = message.command::<Pong>().unwrap();

        assert_eq!("test.host.com", host);
    }

    #[test]
    fn test_privmsg_command() {
        let message = priv_msg("#channel", "This is a message!").unwrap();
        let PrivMsg(target, message) = message.command::<PrivMsg>().unwrap();

        assert_eq!("#channel", target);
        assert_eq!("This is a message!", message);
    }

    #[test]
    fn test_welcome_command() {
        let msg = welcome("robots", "our overlords").unwrap();
        let Welcome(username, message) = msg.command::<Welcome>().unwrap();

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);
    }

    #[test]
    fn test_your_host_command() {
        let msg = your_host("robots", "our overlords").unwrap();
        let YourHost(username, message) = msg.command::<YourHost>().unwrap();

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);
    }

    #[test]
    fn test_created_command() {
        let msg = created("robots", "our overlords").unwrap();
        let Created(username, message) = msg.command::<Created>().unwrap();

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);
    }

    #[test]
    fn test_server_info_command() {
        let msg = server_info("robots", "our overlords").unwrap();
        let ServerInfo(username, message) = msg.command::<ServerInfo>().unwrap();

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);
    }
}
