extern crate regex;
extern crate irc;
extern crate scraper;
extern crate hyper;
extern crate quick_xml;
extern crate rustc_serialize;

use regex::Regex;
use irc::client::prelude::{IrcServer, Server, ServerExt, Config, Command};
use hyper::client::Client;
use hyper::error::Error as HyperError;
use scraper::{Html, Selector};
use std::io::{Read, Error, ErrorKind};

static CHANNEL: &'static str = "#vnluser";
static NAME: &'static str = "luser";
static APPID: &'static str = "ABCDE";

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
                Err(e) => println!("{} {:?}", url, e),
            }
        }
        let wa_regex = Regex::new(r"^!wa (.+)$").unwrap();
        if let Some(input) = wa_regex.captures(line)
                                     .and_then(|caps| caps.at(1)) {
            match wa_query(input) {
                Err(e) => println!("{} {:?}", input, e),
                Ok(text) => {
                    freenode.send(Command::PRIVMSG(CHANNEL.to_owned(), text))
                            .unwrap()
                }
            }
        }
        let google_regex = Regex::new(r"^.g (.+)$").unwrap();
        if let Some(input) = google_regex.captures(line)
                                         .and_then(|caps| caps.at(1)) {
            match google(input) {
                Err(e) => println!("{} {:?}", input, e),
                Ok(text) => freenode.send(Command::PRIVMSG(CHANNEL.to_owned(), text)).unwrap(),
            }
        }
    }
}

fn scrape_title(url: &str) -> Result<String, HyperError> {
    use hyper::header::{UserAgent, Cookie, CookiePair};

    let mut res = try!(Client::new()
                           .get(url)
                           .header(UserAgent("Firefox".to_owned()))
                           .header(Cookie(vec![CookiePair::new(// cookie to access NYtimes articles
                                                               "NYT-S".to_owned(),
                                                               "0MCHCWA5RI93zDXrmvxADeHLKZwNY\
                                                                F3ivqdeFz9JchiAIUFL2BEX5FWcV.\
                                                                Ynx4rkFI"
                                                                   .to_owned())]))
                           .send());
    let mut body = [0; 32768];
    match res.read_exact(&mut body) {
        _ => {}
    };
    match Html::parse_fragment(&String::from_utf8_lossy(&body))
              .select(&Selector::parse("title").unwrap())
              .next() {
        Some(title_elem) => {
            Ok(title_elem.first_child()
                         .unwrap()
                         .value()
                         .as_text()
                         .unwrap()
                         .replace("\n", " ")
                         .trim()
                         .to_owned())
        }
        None => {
            Err(HyperError::Io(Error::new(ErrorKind::InvalidData, "Response doesn't have a title")))
        }
    }
}

fn wa_query(input: &str) -> Result<String, HyperError> {
    use hyper::header::ContentLength;
    use quick_xml::{XmlReader, Event};

    let client = Client::new();
    let mut res = try!(client.get(&format!("http://api.wolframalpha.\
                                            com/v2/query?format=plaintext&appid={}&input={}",
                                           APPID,
                                           input))
                             .send());
    let mut xml = String::with_capacity(**res.headers.get::<ContentLength>().unwrap() as usize);
    try!(res.read_to_string(&mut xml));
    let tree = XmlReader::from_str(&xml).trim_text(true);
    let mut answers = String::new();
    for t in tree {
        if let Ok(Event::Text(e)) = t {
            answers.push_str(&format!("{} ", e.into_string().unwrap()))
        }
    }
    Ok(answers)
}

fn google(input: &str) -> Result<String, HyperError> {
    use rustc_serialize::json::Json;
    // API: https://developers.google.com/web-search/docs/#code-snippets
    let mut res = try!(Client::new()
                           .get(&format!("https://ajax.googleapis.\
                                          com/ajax/services/search/web?v=1.0&rsz=1&q={}",
                                         input))
                           .send());
    let json = Json::from_reader(&mut res).unwrap();
    let ref result = json.search("results").unwrap()[0];
    let url = result.find("unescapedUrl").unwrap();
    let title = result.find("titleNoFormatting").unwrap();
    Ok(format!("{} {}", url, title))
}
