use bytes::Bytes;

fn main() {
    loop {
        honggfuzz::fuzz!(|data: &[u8]| {
            let bytes = Bytes::copy_from_slice(data);
            eprintln!("{:?}", pircolate::message::Message::try_from(bytes));
        });
    }
}
