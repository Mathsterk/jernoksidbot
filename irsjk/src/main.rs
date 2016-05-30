extern crate irc;

use std::default::Default;
use irc::client::prelude::*;

fn main() {
    let config = Config {
        nickname: Some(format!("JernOksid")),
        alt_nicks: Some(vec![format!("IronOxide")]),
        server: Some(format!("irc.efnet.org")),
        channels: Some(vec![format!("#mathsterk")]),
        port: Some(6697),
        use_ssl: Some(true),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => if msg.contains("pickles") {
                server.send_privmsg(target, "Hi!").unwrap();
                println!("{},{}", target, msg);
            },
            _ => (),
        }
    }
}