extern crate core;
extern crate regex;
extern crate irc;
extern crate scraper;
extern crate hyper;

use regex::Regex;
use irc::client::prelude::{IrcServer, Server, ServerExt, Config, Command};

static CHANNEL: &'static str = "#vnluser";
static NAME: &'static str = "luser";

fn main() {
    let freenode = IrcServer::from_config(Config {
                       owners: Some(vec!["hdhoang".to_owned()]),
                       nickname: Some(NAME.to_owned()),
                       server: Some("chat.freenode.net".to_owned()),
                       port: Some(8000),
                       channels: Some(vec![CHANNEL.to_owned()]),
                       ..Default::default()
                   })
                       .unwrap();
    freenode.identify().unwrap();

    for message in freenode.iter() {
        let msg = message.unwrap();
        let line;
        // ignore empty messages
        match msg.suffix {
            None => continue,
            Some(ref l) => line = l,
        }
        if msg.args.get(0) != Some(&CHANNEL.to_owned()) {
            continue;
        }
        // ignore other bots
        if let Some(ref user) = msg.prefix {
            if user.starts_with(NAME) || user.contains("bot") {
                continue;
            }
        }
        let url_regex = Regex::new(r"https?:[^\s]+").unwrap();
        if let Some(url) = url_regex.captures(line)
                                    .and_then(|caps| caps.at(0)) {
            match scrape_title(url) {
                Ok(title) => {
                    freenode.send(Command::PRIVMSG(CHANNEL.to_owned(),
                                                   format!("TITLE: {}", title)))
                            .unwrap();
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }
}

fn scrape_title(url: &str) -> Result<String, hyper::error::Error> {
    use hyper::client::Client;
    use hyper::header::{UserAgent, ContentType, Cookie, CookiePair};
    use hyper::mime::{Mime, TopLevel};
    use scraper::{Html, Selector};
    use std::io::Read;
    use core::ops::Deref;

    let select_title = Selector::parse("title").unwrap();
    let client = Client::new();
    let res = try!(client.get(url)
                         .header(UserAgent("Firefox".to_owned()))
                         .header(Cookie(vec![CookiePair::new(// cookie to access NYtimes articles
                                                             "NYT-S".to_owned(),
                                                             "0MCHCWA5RI93zDXrmvxADeHLKZwNYF3\
                                                              ivqdeFz9JchiAIUFL2BEX5FWcV.\
                                                              Ynx4rkFI"
                                                                 .to_owned())]))
                         .send());
    match res.headers.get::<ContentType>() {
        Some(&ContentType(Mime(TopLevel::Text, _, _))) => {
            let mut html = String::with_capacity(32768);
            try!(res.take(32768).read_to_string(&mut html));
            Ok(Html::parse_fragment(&html)
                   .select(&select_title)
                   .next()
                   .unwrap()
                   .first_child()
                   .unwrap()
                   .value()
                   .as_text()
                   .unwrap()
                   .deref()
                   .replace("\n", " ")
                   .to_owned())
        }
        _ => Ok("".to_owned()),
    }
}
