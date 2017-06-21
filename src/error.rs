// TODO: Come back and document this.
#![allow(missing_docs)]

error_chain! {
    foreign_links {
        Utf8(::std::string::FromUtf8Error);
    }

    errors { 
        UnexpectedEndOfInput {
            description("Encountered unexpected end of input while reading message from server.")
            display("Encountered unexpected end of input while reading message from server.")
        }

        InputTooLong(message: String) {
            description("The input was too long.")
            display("{}", message)
        }
    }
}
