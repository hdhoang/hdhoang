extern crate regex;
extern crate irc;
extern crate scraper;
extern crate hyper;
extern crate quick_xml;
extern crate rustc_serialize;
extern crate url;

use regex::Regex;
use irc::client::prelude::{IrcServer, Server, ServerExt, Config, Command, Response};
use hyper::client::Client;
use std::io::Read;

const CHANNEL: &'static str = "#vnluser";
const NAME: &'static str = "luser";

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
    let freenode = IrcServer::from_config(Config {
                       owners: Some(vec!["hdhoang".into()]),
                       username: Some(NAME.into()),
                       nickname: Some(format!("{}-0", NAME)),
                       alt_nicks: Some((1..10).map(|n| format!("{}-{}", NAME, n)).collect()),
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
            if !lusers.contains(&freenode.current_nickname().into()) {
                let _ = freenode.reconnect();
            }
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
                                           format!("operated by {} with source code {}",
                                                   freenode.config()
                                                           .owners
                                                           .as_ref()
                                                           .map(|v| v.join(", "))
                                                           .unwrap_or("someone anonymous"
                                                                          .into()),
                                                   post_source_code())))
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

    let url = regex.captures(&line).unwrap().expand("$0");
    if ["imgur.com/", "smbc-comics.com/"].iter().any(|uninteresting| url.contains(uninteresting)) {
        return Ok(String::new());
    }
    let mut response = try!(Client::new()
                           .get(&url)
                           .header(UserAgent("Firefox".into()))
                           .header(Cookie(vec![CookiePair::new(// cookie to access NYtimes articles
                                                               "NYT-S".into(),
                                                               "0MCHCWA5RI93zDXrmvxADeHLKZwNY\
                                                                F3ivqdeFz9JchiAIUFL2BEX5FWcV.\
                                                                Ynx4rkFI"
                                                                   .into())]))
                           .send()
                           .map_err(Error::Hyper));
    let mut body = [0; 50_000];
    response.read_exact(&mut body).ok();
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

    let mut response = try!(Client::new()
                                .get(&regex.captures(&line)
                                           .unwrap()
                                           .expand(&format!("http://api.wolframalpha.\
                                                             com/v2/query?format=plaintext&app\
                                                             id={}&input=$query",
                                                            include_str!("wolframalpha_key"))))
                                .send()
                                .map_err(Error::Hyper));
    let mut xml =
        String::with_capacity(**response.headers.get::<ContentLength>().unwrap() as usize);
    try!(response.read_to_string(&mut xml).map_err(Error::Io));
    let tree = XmlReader::from_str(&xml).trim_text(true);
    let mut answers = vec![];
    for t in tree {
        if let Ok(Event::Text(e)) = t {
            answers.push(try!(e.into_string().map_err(Error::Xml)))
        }
    }
    Ok(answers.join(" | "))
}

const GOOGLE_REGEX: &'static str = concat!(r"^(\.|!|:)", "g (?P<query>.+)");
fn google(regex: &Regex, line: &str) -> Result<String, Error> {
    use rustc_serialize::json::Json;
    // API: https://developers.google.com/web-search/docs/#code-snippets
    let mut response = try!(Client::new()
                                .get(&regex.captures(&line)
                                           .unwrap()
                                           .expand("https://ajax.googleapis.\
                                                    com/ajax/services/search/web?v=1.\
                                                    0&rsz=1&q=$query"))
                                .send()
                                .map_err(Error::Hyper));
    let json = try!(Json::from_reader(&mut response).map_err(Error::Json));
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
    let mut response = try!(Client::new()
                                .get(&regex.captures(&line)
                                           .unwrap()
                                           .expand(&format!("https://translate.yandex.\
                                                             net/api/v1.5/tr.\
                                                             json/translate?key={}&text=$text&\
                                                             lang=$lang",
                                                            include_str!("yandex_key"))))
                                .send()
                                .map_err(Error::Hyper));
    let json = try!(Json::from_reader(&mut response).map_err(Error::Json));
    let reply = match json.find("code").unwrap().as_u64().unwrap() {
        200 => {
            format!("{}: {}",
                    json.find("lang").unwrap().as_string().unwrap(),
                    json.find("text").unwrap()[0].as_string().unwrap())
        }
        501 => json.find("message").unwrap().as_string().unwrap().into(),
        _ => format!("{:?}", json.as_string()),
    };
    Ok(reply)
}

fn post_source_code() -> String {
    use url::form_urlencoded;
    let form = [("read:1", "3"),
                ("name:1", "main.rs"),
                ("f:1", include_str!("main.rs")),
                ("read:2", "3"),
                ("name:2", "Cargo.toml"),
                ("f:2", include_str!("../Cargo.toml"))];
    let result = Client::new()
                     .post("http://ix.io")
                     .body(&form_urlencoded::serialize(form.iter()))
                     .send();
    match result {
        Ok(mut response) => {
            let mut reply = String::new();
            let _ = response.read_to_string(&mut reply);
            reply
        }
        Err(e) => format!("unable to post: {:?}", e),
    }
}
