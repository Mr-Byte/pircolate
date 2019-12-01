#![feature(test)]
#![warn(rust_2018_idioms)]

extern crate test;

use pircolate::Message;
use test::Bencher;

#[bench]
fn parse_ping(bencher: &mut Bencher) {
    bencher.iter(|| {
        Message::try_from("PING :test.host.com").unwrap();
    })
}

#[bench]
fn parse_long_message(bencher: &mut Bencher) {
    bencher.iter(|| {
        Message::try_from(
            "@badge-info=subscriber/2;badges=subscriber/0,premium/1;color=#1600A8;display-name=WaffleToppington;emotes=300930428:0-7,17-24,34-41/300105444:9-15,26-32;flags=;id=4d1b8457-7cf9-4a6d-9d68-af6b78686b6d;mod=0;room-id=121059319;subscriber=1;tmi-sent-ts=1575098616559;turbo=0;user-id=90565271;user-type= :waffletoppington!waffletoppington@waffletoppington.tmi.twitch.tv PRIVMSG #moonmoon :moon2DEV moon2GN moon2DEV moon2GN moon2DEV").unwrap();
    })
}
