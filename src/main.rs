extern crate regex;
extern crate irc;
extern crate scraper;
extern crate hyper;
extern crate quick_xml;
extern crate rustc_serialize;

use regex::Regex;
use irc::client::prelude::{IrcServer, Server, ServerExt, Config, Command, Response};
use hyper::client::Client;
use std::io::Read;

static CHANNEL: &'static str = "#vnluser";
static NAME: &'static str = "luser";
static APPID: &'static str = "3JEW42-4XXE264A93";
static YANDEX_KEY: &'static str = "trnsl.1.1.20160210T093900Z.c6eacf09bbb65cfb.\
                                   cc28de2ba798bc3bc118e9f8201b6e6cea697810";

#[derive(Debug)]
enum Error {
    Data(String),
    Io(std::io::Error),
    Hyper(hyper::error::Error),
    Xml(quick_xml::error::Error),
    Json(rustc_serialize::json::ParserError),
}

struct Handler<'a>(Regex, &'a (Fn(&Regex, &str) -> Result<String, Error>));
impl<'a> Handler<'a> {
    fn can_handle(&self, line: &str) -> bool {
        self.0.is_match(&line)
    }
    fn run(&self, line: &str) -> Result<String, Error> {
        self.1(&self.0, &line)
    }
}

fn main() {
    let mut alt_nicks = vec![];
    for n in 1..10 {
        alt_nicks.push(format!("{}-{}", NAME, n))
    }

    let freenode = IrcServer::from_config(Config {
                       owners: Some(vec!["hdhoang".to_owned()]),
                       nickname: Some(format!("{}-0", NAME)),
                       alt_nicks: Some(alt_nicks),
                       server: Some("chat.freenode.net".to_owned()),
                       port: Some(8000),
                       channels: Some(vec![String::from(CHANNEL), format!("#{}-test", NAME)]),
                       ..Default::default()
                   })
                       .unwrap();
    freenode.identify().unwrap();

    let get_title = &get_title;
    let wolframalpha = &wolframalpha;
    let google = &google;
    let translate = &translate;
    let handlers = [Handler(Regex::new(TITLE_REGEX).unwrap(), get_title),
                    Handler(Regex::new(WA_REGEX).unwrap(), wolframalpha),
                    Handler(Regex::new(GOOGLE_REGEX).unwrap(), google),
                    Handler(Regex::new(TRANSLATE_REGEX).unwrap(), translate)];
    let report_regex = Regex::new("^report!$").unwrap();

    let mut lusers = vec![];
    'messages: for message in freenode.iter() {
        let msg = message.unwrap();

        // Get other lusers
        if let Command::Response(Response::RPL_NAMREPLY, _, Some(ref names)) = msg.command {
            lusers.extend(names.split(' ')
                               .filter(|n| n.starts_with(NAME))
                               .map(String::from));
            lusers.sort();
            lusers.dedup();
            continue 'messages;
        }
        if let Some(ref nick) = msg.source_nickname() {
            // Ignore bots and freenode
            if nick.contains("bot") || nick.contains("freenode") {
                continue 'messages;
            }
            // Update lusers list
            if nick.starts_with(NAME) {
                match msg.command {
                    Command::JOIN(_, _, _) => {
                        lusers.push(String::from(*nick));
                        lusers.sort();
                        lusers.dedup();
                    }
                    Command::QUIT(_) => {
                        if let Ok(idx) = lusers.binary_search(&String::from(*nick)) {
                            println!("{} quitted, leaving {:?}.", lusers.remove(idx), lusers)
                        }
                    }
                    _ => (),
                }
                continue 'messages;
            }
        }

        let channel;
        let line;
        match msg.command {
            Command::PRIVMSG(ref target, ref message) => {
                channel = target;
                line = message
            }
            _ => continue 'messages,

        }

        if report_regex.is_match(line) {
            freenode.send(Command::PRIVMSG(channel.clone(),
                                           format!("{} operated by {}",
                                                   freenode.current_nickname(),
                                                   freenode.config()
                                                           .owners
                                                           .as_ref()
                                                           .map(|v| v.join(", "))
                                                           .unwrap_or("someone".to_owned()))))
                    .unwrap();
            continue 'messages;
        }
        // Don't response if we're not at the front
        if lusers[0] != freenode.current_nickname() {
            continue 'messages;
        }

        'handling: for h in &handlers {
            if h.can_handle(&line) {
                match h.run(&line) {
                    Err(e) => println!("{:?} causes {:?}", line, e),
                    Ok(reply) => {
                        if !reply.is_empty() {
                            freenode.send(Command::PRIVMSG(channel.clone(), reply)).unwrap();
                            continue 'messages;
                        }
                    }
                }
            }
        }
    }
}

static TITLE_REGEX: &'static str = r"https?:[^\s]+";
fn get_title(regex: &Regex, line: &str) -> Result<String, Error> {
    use hyper::header::{UserAgent, Cookie, CookiePair};
    use scraper::{Html, Selector};

    let mut res = try!(Client::new()
                           .get(&regex.captures(&line)
                                      .unwrap()
                                      .expand("$0"))
                           .header(UserAgent("Firefox".to_owned()))
                           .header(Cookie(vec![CookiePair::new(// cookie to access NYtimes articles
                                                               "NYT-S".to_owned(),
                                                               "0MCHCWA5RI93zDXrmvxADeHLKZwNY\
                                                                F3ivqdeFz9JchiAIUFL2BEX5FWcV.\
                                                                Ynx4rkFI"
                                                                   .to_owned())]))
                           .send()
                           .map_err(Error::Hyper));
    let mut body = [0; 32768];
    match res.read_exact(&mut body) {
        _ => {}
    };
    match Html::parse_fragment(&String::from_utf8_lossy(&body))
              .select(&Selector::parse("title").unwrap())
              .next() {
        Some(title_elem) => {
            Ok(format!("TITLE: {}",
                       title_elem.first_child()
                                 .unwrap()
                                 .value()
                                 .as_text()
                                 .unwrap()
                                 .replace("\n", " ")
                                 .trim()))
        }
        None => Err(Error::Data("Response doesn't have a title".to_owned())),
    }
}

static WA_REGEX: &'static str = r"^.wa (?P<query>.+)";
fn wolframalpha(regex: &Regex, line: &str) -> Result<String, Error> {
    use hyper::header::ContentLength;
    use quick_xml::{XmlReader, Event};

    let mut res = try!(Client::new()
                           .get(&regex.captures(&line)
                                      .unwrap()
                                      .expand(&format!("http://api.wolframalpha.\
                                                        com/v2/query?format=plaintext&appid={}\
                                                        &input=$query",
                                                       APPID)))
                           .send()
                           .map_err(Error::Hyper));
    let mut xml = String::with_capacity(**res.headers.get::<ContentLength>().unwrap() as usize);
    try!(res.read_to_string(&mut xml).map_err(Error::Io));
    let tree = XmlReader::from_str(&xml).trim_text(true);
    let mut answers = String::new();
    for t in tree {
        if let Ok(Event::Text(e)) = t {
            answers.push_str(&format!("{} | ", try!(e.into_string().map_err(Error::Xml))))
        }
    }
    Ok(answers)
}

static GOOGLE_REGEX: &'static str = r"^.g (?P<query>.+)";
fn google(regex: &Regex, line: &str) -> Result<String, Error> {
    use rustc_serialize::json::Json;
    // API: https://developers.google.com/web-search/docs/#code-snippets
    let mut res = try!(Client::new()
                           .get(&regex.captures(&line)
                                      .unwrap()
                                      .expand("https://ajax.googleapis.\
                                               com/ajax/services/search/web?v=1.\
                                               0&rsz=1&q=$query"))
                           .send()
                           .map_err(Error::Hyper));
    let json = try!(Json::from_reader(&mut res).map_err(Error::Json));
    let results = try!(json.search("results").ok_or(Error::Data("No results".to_owned())));
    if results.as_array().unwrap().is_empty() {
        return Ok("No results".to_owned());
    }
    let url = try!(results[0]
                       .find("unescapedUrl")
                       .ok_or(Error::Data("No url".to_owned()))
                       .map(|j| j.as_string().unwrap()));
    let title = try!(results[0]
                         .find("titleNoFormatting")
                         .ok_or(Error::Data("No title".to_owned()))
                         .map(|j| j.as_string().unwrap()));
    Ok(format!("{} {}", title, url))
}

static TRANSLATE_REGEX: &'static str = r"^.tr (?P<lang>[^ ]+) (?P<text>.+)";
fn translate(regex: &Regex, line: &str) -> Result<String, Error> {
    use rustc_serialize::json::Json;
    // API: https://tech.yandex.com/translate/doc/dg/reference/translate-docpage/
    let mut res = try!(Client::new()
                           .get(&regex.captures(&line)
                                      .unwrap()
                                      .expand(&format!("https://translate.yandex.net/api/v1.\
                                                        5/tr.json/translate?key={}&text=$text&\
                                                        lang=$lang",
                                                       YANDEX_KEY)))
                           .send()
                           .map_err(Error::Hyper));
    let json = try!(Json::from_reader(&mut res).map_err(Error::Json));
    let response = match json.find("code").unwrap().as_u64().unwrap() {
        200 => {
            format!("{}: {}",
                    json.find("lang").unwrap().as_string().unwrap(),
                    json.find("text").unwrap()[0].as_string().unwrap())
        }
        501 => json.find("message").unwrap().as_string().unwrap().to_owned(),
        _ => json.as_string().unwrap().to_owned(),
    };
    Ok(response)
}
