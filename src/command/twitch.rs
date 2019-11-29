use super::*;
use crate::{command, expand_param};

command! {
    /// Represents a PING command.  The first element is the host.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate pircolate;
    /// # use pircolate::message;
    /// # use pircolate::command::Ping;
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() {
    /// # let msg = message::Message::try_from("PING :test.host.com").unwrap();
    /// if let Some(Ping(host)) = msg.command::<Ping>() {
    ///     println!("PING from {}", host);
    /// }
    /// # }
    /// ```
    ("PING" => Ping(host))
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
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() {
    /// # let msg = message::Message::try_from("PONG :test.host.com").unwrap();
    /// if let Some(Pong(host)) = msg.command::<Pong>() {
    ///    println!("PONG from {}.", host);
    /// }
    /// # }
    /// ```
    ("PONG" => Pong(host))
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
    /// # use std::convert::TryFrom;
    /// #
    /// # fn main() {
    /// # let msg = message::Message::try_from("PRIVMSG memelord :memes are great").unwrap();
    /// if let Some(PrivMsg(user, message)) = msg.command::<PrivMsg>() {
    ///     println!("<{}> {}.", user, message);
    /// }
    /// # }
    /// ```
    ("PRIVMSG" => PrivMsg(target, message))
}

command! {
    ("JOIN" => Join(channel))
}

command! {
    /// Represents a WELCOME numeric. The first element is the unsername and the second element is the welcome message.
    ("001" => Welcome(user, message))
}

command! {
    /// Represents a YOURHOST numeric. The first element is the unsername and the second element is the yourhost message.
    ("002" => YourHost(user, message))
}

command! {
    /// Represents a CREATED numeric. The first element is the unsername and the second element is the created message.
    ("003" => Created(user, message))
}

command! {
    /// Represents a MYINFO numeric. The first element is the username and the second element is the server info message.
    ("004" => ServerInfo(user, message))
}

#[derive(PartialEq, Debug)]
pub enum NamesReplyChannelType {
    Secret,
    Private,
    Other,
}

pub struct NamesReply<'a>(pub NamesReplyChannelType, pub &'a str, pub Vec<&'a str>);

impl<'a> Command<'a> for NamesReply<'a> {
    const NAME: &'static str = "353";

    fn parse(arguments: ArgumentIter<'a>) -> Option<NamesReply<'a>> {
        // NOTE: Since the first parameter is optional, it's just easier to extract
        // components in reverse.
        let mut arguments = arguments.rev();

        let names = arguments.next()?.split_whitespace();
        let channel = arguments.next()?;
        let channel_type = match arguments.next() {
            Some(channel_type) => match channel_type {
                "@" => NamesReplyChannelType::Secret,
                "*" => NamesReplyChannelType::Private,
                _ => NamesReplyChannelType::Other,
            },
            None => NamesReplyChannelType::Other,
        };

        Some(NamesReply(channel_type, channel, names.collect()))
    }
}

pub struct EndNamesReply<'a>(pub &'a str, pub &'a str);

impl<'a> Command<'a> for EndNamesReply<'a> {
    const NAME: &'static str = "366";

    fn parse(arguments: ArgumentIter<'a>) -> Option<EndNamesReply<'a>> {
        // NOTE: Some servers are bad and include non-standard args at the start.
        // So the parameters are extracted in reverse to compensate.
        let mut arguments = arguments.rev();

        let message = arguments.next()?;
        let channel = arguments.next()?;

        Some(EndNamesReply(channel, message))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Message;
    use anyhow::{Context, Result};
    use std::convert::TryFrom;

    #[test]
    fn test_ping_command() -> Result<()> {
        let message: Message = Message::try_from("PING :test.host.com")?;
        let Ping(host) = message.command().context("Invalid ping command.")?;

        assert_eq!("test.host.com", host);
        Ok(())
    }

    #[test]
    fn test_pong_command() -> Result<()> {
        let message: Message = Message::try_from("PONG :test.host.com")?;
        let Pong(host) = message.command().context("Invalid pong command.")?;

        assert_eq!("test.host.com", host);
        Ok(())
    }

    #[test]
    fn test_privmsg_command() -> Result<()> {
        let message: Message = Message::try_from("PRIVMSG #channel :This is a message!")?;
        let PrivMsg(target, message) = message.command().context("Invalid privmsg command.")?;

        assert_eq!("#channel", target);
        assert_eq!("This is a message!", message);
        Ok(())
    }

    #[test]
    fn test_welcome_command() -> Result<()> {
        let msg: Message = Message::try_from("001 robots :our overlords")?;
        let Welcome(username, message) = msg.command().context("Invalid welcome command.")?;

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);

        Ok(())
    }

    #[test]
    fn test_your_host_command() -> Result<()> {
        let msg: Message = Message::try_from("002 robots :our overlords")?;
        let YourHost(username, message) = msg.command().context("Invalid your host command.")?;

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);

        Ok(())
    }

    #[test]
    fn test_created_command() -> Result<()> {
        let msg: Message = Message::try_from("003 robots :our overlords")?;
        let Created(username, message) = msg.command().context("Invalid created command.")?;

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);

        Ok(())
    }

    #[test]
    fn test_server_info_command() -> Result<()> {
        let msg: Message = Message::try_from("004 robots :our overlords")?;
        let ServerInfo(username, message) =
            msg.command().context("Invalid server info command.")?;

        assert_eq!("robots", username);
        assert_eq!("our overlords", message);

        Ok(())
    }

    #[test]
    fn test_names_reply_command() -> Result<()> {
        let msg: Message = Message::try_from("353 = #test :robot1 robot2 robot3")?;
        let NamesReply(channel_type, channel, users) =
            msg.command().context("Invaid names reply command.")?;

        let expected_users = vec!["robot1", "robot2", "robot3"];

        assert_eq!(NamesReplyChannelType::Other, channel_type);
        assert_eq!("#test", channel);
        assert_eq!(expected_users, users);

        Ok(())
    }
}
