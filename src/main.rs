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

const CHANNEL: &'static str = "#vnluser";
const NAME: &'static str = "luser";
const APPID: &'static str = "3JEW42-4XXE264A93";
const YANDEX_KEY: &'static str = "trnsl.1.1.20160210T093900Z.c6eacf09bbb65cfb.\
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
                       nickname: Some(format!("{}-0", NAME)),
                       alt_nicks: Some(alt_nicks),
                       server: Some("chat.freenode.net".into()),
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
        if let Some(nick) = msg.source_nickname() {
            // Ignore bots and freenode
            if nick.contains("bot") || nick.contains("freenode") {
                continue 'messages;
            }
            if nick.starts_with(NAME) {
                let nick = String::from(nick);
                // Update lusers list
                match msg.command {
                    // Do not merge the following arms
                    // Otherwise join -> insert -> join -> remove might happen
                    Command::JOIN(..) => {
                        if let Err(idx) = lusers.binary_search(&nick) {
                            lusers.insert(idx, nick)
                        }
                    }
                    Command::QUIT(..) => {
                        if let Ok(idx) = lusers.binary_search(&nick) {
                            lusers.remove(idx);
                        }
                    }
                    _ => (),
                }
                // Ignore them
                continue 'messages;
            }
        }

        let channel;
        let line;
        if let Command::PRIVMSG(ref target, ref message) = msg.command {
            channel = target;
            line = message
        } else {
            continue 'messages;
        }

        if line == "report!" {
            freenode.send(Command::PRIVMSG(channel.clone(),
                                           format!("operated by {}",
                                                   freenode.config()
                                                           .owners
                                                           .as_ref()
                                                           .map(|v| v.join(", "))
                                                           .unwrap_or("someone anonymous"
                                                                          .into()))))
                    .unwrap();
            continue 'messages;
        }
        if lusers[msg.prefix.unwrap().len() % lusers.len()] != freenode.current_nickname() {
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

const TITLE_REGEX: &'static str = r"https?:[^\s]+";
fn get_title(regex: &Regex, line: &str) -> Result<String, Error> {
    use hyper::header::{UserAgent, Cookie, CookiePair};
    use scraper::{Html, Selector};

    let mut res = try!(Client::new()
                           .get(&regex.captures(&line)
                                      .unwrap()
                                      .expand("$0"))
                           .header(UserAgent("Firefox".into()))
                           .header(Cookie(vec![CookiePair::new(// cookie to access NYtimes articles
                                                               "NYT-S".into(),
                                                               "0MCHCWA5RI93zDXrmvxADeHLKZwNY\
                                                                F3ivqdeFz9JchiAIUFL2BEX5FWcV.\
                                                                Ynx4rkFI"
                                                                   .into())]))
                           .send()
                           .map_err(Error::Hyper));
    let mut body = [0; 32768];
    res.read_exact(&mut body).ok();
    if let Some(title_elem) = Html::parse_fragment(&String::from_utf8_lossy(&body))
                                  .select(&Selector::parse("title").unwrap())
                                  .next() {
        Ok(title_elem.first_child()
                     .unwrap()
                     .value()
                     .as_text()
                     .unwrap()
                     .replace("\n", " ")
                     .trim()
                     .into())
    } else {
        Err(Error::Data("Response has no title".into()))
    }
}

const WA_REGEX: &'static str = concat!(r"^(\.|!|:)", "wa (?P<query>.+)");
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

const GOOGLE_REGEX: &'static str = concat!(r"^(\.|!|:)", "g (?P<query>.+)");
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
    let results = try!(json.search("results").ok_or(Error::Data("No results".into())));
    if results.as_array().unwrap().is_empty() {
        return Ok("No results".into());
    }
    let url = try!(results[0]
                       .find("unescapedUrl")
                       .ok_or(Error::Data("No url".into()))
                       .map(|j| j.as_string().unwrap()));
    let title = try!(results[0]
                         .find("titleNoFormatting")
                         .ok_or(Error::Data("No title".into()))
                         .map(|j| j.as_string().unwrap()));
    Ok(format!("{} {}", title, url))
}

const TRANSLATE_REGEX: &'static str = concat!(r"^(\.|!|:)", "tr (?P<lang>[^ ]+) (?P<text>.+)");
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
        501 => json.find("message").unwrap().as_string().unwrap().into(),
        _ => json.as_string().unwrap().into(),
    };
    Ok(response)
}
