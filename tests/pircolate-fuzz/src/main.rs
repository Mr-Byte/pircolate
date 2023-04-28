fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            let message = pircolate::message::Message::try_from(data);
            if let Ok(message) = message {
                let _message_clone = message.clone();
            }
        });
    }
}
