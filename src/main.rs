extern crate regex;
extern crate irc;
extern crate scraper;
extern crate hyper;
extern crate quick_xml;
extern crate rustc_serialize;

use regex::Regex;
use irc::client::prelude::{IrcServer, Server, ServerExt, Config, Command};
use hyper::client::Client;
use scraper::{Html, Selector};
use std::io::Read;

static CHANNEL: &'static str = "#vnluser";
static NAME: &'static str = "luser";
static APPID: &'static str = "ABCDE";

fn main() {
    let freenode = IrcServer::from_config(Config {
                       owners: Some(vec!["hdhoang".to_owned()]),
                       nickname: Some(NAME.to_owned()),
                       alt_nicks: Some(vec![format!("{}-dev", NAME)]),
                       server: Some("chat.freenode.net".to_owned()),
                       port: Some(8000),
                       channels: Some(vec![CHANNEL.to_owned(), "#luser-test".to_owned()]),
                       ..Default::default()
                   })
                       .unwrap();
    freenode.identify().unwrap();

    let mut handlers: Vec<Box<Handler>> = vec![];
    handlers.push(Box::new(Titler { regex: Regex::new(r"https?:[^\s]+").unwrap() }));
    handlers.push(Box::new(WolframAlpha { regex: Regex::new(r"^.wa (.+)").unwrap() }));
    handlers.push(Box::new(Google { regex: Regex::new(r"^.g (.+)").unwrap() }));

    for message in freenode.iter() {
        let msg = message.unwrap();
        let line;
        // ignore empty messages
        match msg.suffix {
            None => continue,
            Some(ref l) => line = l,
        }
        let channel;
        match msg.args.get(0) {
            None => continue,
            Some(c) => channel = c,
        }
        // ignore other bots
        if let Some(ref user) = msg.prefix {
            if user.starts_with(NAME) || user.contains("bot") {
                continue;
            }
        }

        for h in &handlers {
            match h.run(&line) {
                Err(e) => println!("{} {:?}", line, e),
                Ok(reply) => {
                    if reply.len() > 0 {
                        freenode.send(Command::PRIVMSG(channel.clone(), reply)).unwrap()
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum Error {
    Data(String),
    Io(std::io::Error),
    Hyper(hyper::error::Error),
    Xml(quick_xml::error::Error),
    Json(rustc_serialize::json::ParserError),
}

trait Handler {
    fn run(&self, line: &String) -> Result<String, Error>;
}

struct Titler {
    regex: Regex,
}
impl Handler for Titler {
    fn run(&self, line: &String) -> Result<String, Error> {
        if let Some(url) = self.regex.captures(line).and_then(|caps| caps.at(0)) {
            use hyper::header::{UserAgent, Cookie, CookiePair};

            let mut res = try!(Client::new()
                         .get(url)
                         .header(UserAgent("Firefox".to_owned()))
                         .header(Cookie(vec![CookiePair::new(// cookie to access NYtimes articles
                                                             "NYT-S".to_owned(),
                                                             "0MCHCWA5RI93zDXrmvxADeHLKZwNYF3\
                                                              ivqdeFz9JchiAIUFL2BEX5FWcV.\
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
        } else {
            Ok(String::new())
        }
    }
}

struct WolframAlpha {
    regex: Regex,
}
impl Handler for WolframAlpha {
    fn run(&self, line: &String) -> Result<String, Error> {
        if let Some(input) = self.regex.captures(line).and_then(|caps| caps.at(1)) {
            use hyper::header::ContentLength;
            use quick_xml::{XmlReader, Event};

            let mut res = try!(Client::new()
                                   .get(&format!("http://api.wolframalpha.\
                                                  com/v2/query?format=plaintext&appid={}&input=\
                                                  {}",
                                                 APPID,
                                                 input))
                                   .send()
                                   .map_err(Error::Hyper));
            let mut xml =
                String::with_capacity(**res.headers.get::<ContentLength>().unwrap() as usize);
            try!(res.read_to_string(&mut xml).map_err(Error::Io));
            let tree = XmlReader::from_str(&xml).trim_text(true);
            let mut answers = String::new();
            for t in tree {
                if let Ok(Event::Text(e)) = t {
                    answers.push_str(&format!("{} ", try!(e.into_string().map_err(Error::Xml))))
                }
            }
            Ok(answers)
        } else {
            Ok(String::new())
        }
    }
}
struct Google {
    regex: Regex,
}
impl Handler for Google {
    fn run(&self, line: &String) -> Result<String, Error> {
        if let Some(input) = self.regex.captures(line).and_then(|caps| caps.at(1)) {
            use rustc_serialize::json::Json;
            // API: https://developers.google.com/web-search/docs/#code-snippets
            let mut res = try!(Client::new()
                                   .get(&format!("https://ajax.googleapis.\
                                                  com/ajax/services/search/web?v=1.0&rsz=1&q={}",
                                                 input))
                                   .send()
                                   .map_err(Error::Hyper));
            let json = try!(Json::from_reader(&mut res).map_err(Error::Json));
            let ref result =
                try!(json.search("results").ok_or(Error::Data("No results".to_owned())))[0];
            let url = try!(result.find("unescapedUrl")
                                 .ok_or(Error::Data("No url".to_owned()))
                                 .map(|j| j.as_string().unwrap()));
            let title = try!(result.find("titleNoFormatting")
                                   .ok_or(Error::Data("No title".to_owned()))
                                   .map(|j| j.as_string().unwrap()));
            Ok(format!("{} {}", url, title))
        } else {
            Ok(String::new())
        }
    }
}
