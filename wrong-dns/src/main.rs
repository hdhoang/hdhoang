use domain::base::iana::rtype::Rtype;
use domain::base::name::Dname;
use domain::base::question::Question;

use domain::resolv::stub::{
    conf::{ResolvConf, ResolvOptions, ServerConf, Transport},
    StubResolver,
};

const USAGE: &str = "Usage: wrong-dns [@ipv4-server|@[ipv6-server]] RTYPE...

Port is always udp/53. Default server is CloudFlare 1.1.1.3";

struct Args {
    server: String,
    rtypes: Vec<Rtype>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    let server = args.server;
    let mut server = ServerConf::new(format!("{server}:53").parse()?, Transport::Udp);
    server.request_timeout = core::time::Duration::from_millis(100);
    let options = ResolvOptions {
        attempts: 0,
        ..Default::default()
    };
    let resolv = StubResolver::from_conf(ResolvConf {
        servers: vec![server],
        options,
    });

    for rtype in args.rtypes {
        print!("{rtype}: ");
        let q = naughty_question(&rtype);
        let res = resolv.query(q).await;
        match res {
            Err(e) if e.kind() == tokio::io::ErrorKind::TimedOut => {
                println!("server ignored us, perhaps: {e:?}");

            }
            Err(e) => Err(e)?,
            Ok(answer) => {
                println!("{:?}", answer.as_octets())
            }
        };
    }
    Ok(())
}

fn naughty_question(qtype: &Rtype) -> Question<Dname<Vec<u8>>> {
    let name = Dname::vec_from_str("example.").unwrap();
    let mut bytes = name.into_octets();
    bytes.push(0);
    Question::new_in(
        // Safety: I want to do it wrong
        unsafe { Dname::from_octets_unchecked(bytes) },
        qtype.clone(),
    )
}
#[test]
fn it_is_different() {
    assert!(
        !Question::from((Dname::vec_from_str("example.").unwrap(), Rtype::Any))
            .eq(&naughty_question(&Rtype::Any))
    );
}

fn parse_args() -> Result<Args, Box<dyn std::error::Error>> {
    use lexopt::prelude::*;

    let mut server = "1.1.1.3".into();
    let mut rtypes = vec![];

    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('h') | Long("help") => {
                eprintln!("{}", USAGE);
                std::process::exit(0);
            }
            Value(val) => {
                let val = val.parse::<String>()?;
                if let Some(ip) = val.strip_prefix('@') {
                    server = ip.to_string();
                } else {
                    rtypes.push(val.parse()?);
                }
            }
            _ => return Err(arg.unexpected().into()),
        }
    }
    if rtypes.is_empty() {
        eprintln!("{}", USAGE);
        std::process::exit(1);
    }
    Ok(Args { server, rtypes })
}
