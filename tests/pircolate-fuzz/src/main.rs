use std::convert::TryFrom;

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            if let Ok(data) = std::str::from_utf8(data) {
                eprintln!("{:?}", pircolate::message::Message::try_from(data));
            }
        });
    }
}
