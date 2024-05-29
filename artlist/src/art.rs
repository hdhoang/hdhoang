#![allow(dead_code)]

use url::Url;

pub type Name = String;
pub type Id = String;

#[derive(Debug, Default, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Words(#[knuffel(arguments)] Vec<String>);

#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(Default, PartialEq))]
pub struct Art {
    /// prominent web handle
    #[knuffel(node_name)]
    handle: Name,
    #[knuffel(child, default)]
    aliases: Words,
    #[knuffel(child, default)]
    styles: Words,

    /// official website
    #[knuffel(argument, default, str)]
    web: Option<Url>,
    /// its feed, and readablility score
    #[knuffel(child)]
    feed: Option<feed::Feed>,

    /// linktr.ee, carrd.co, ... and the like
    #[knuffel(property, str)]
    link_list: Option<Url>,

    // capital pipes
    #[knuffel(child)]
    pat: Option<Patreon>,
    #[knuffel(child)]
    kofi: Option<Kofi>,
    #[knuffel(child)]
    github: Option<Github>,

    // 1-way feed
    #[knuffel(child)]
    tiny: Option<Tinyview>,
    #[knuffel(child)]
    fb: Option<Facebook>,
    #[knuffel(child)]
    go: Option<Gocomics>,
    #[knuffel(child)]
    tumblr: Option<Tumblr>,
    #[knuffel(child)]
    substack: Option<Substack>,
    #[knuffel(child)]
    bsky: Option<Bsky>,
    #[knuffel(child)]
    cohost: Option<Cohost>,
    #[knuffel(child)]
    da: Option<DeviantArt>,
    #[knuffel(child)]
    yt: Option<Youtube>,
    #[knuffel(child)]
    ig: Option<Insta>,

    /// Huginn agent id
    #[knuffel(child)]
    huginn: Option<HuginnId>,

    // interactive flows
    #[knuffel(child)]
    fedi: Option<Fedi>,
    #[knuffel(child)]
    twt: Option<Twitter>,
}

macro_rules! simple_account {
    ($site: ident, $url_template: expr, $feed_template: expr ) => {
        #[derive(Debug, knuffel::Decode)]
        #[cfg_attr(test, derive(PartialEq))]
        pub struct $site {
            #[knuffel(argument)]
            vanity: String,
        }

        impl std::fmt::Display for $site {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter,
            ) -> std::fmt::Result {
                write!(f, $url_template, &self.vanity)
            }
        }
        impl crate::art::feed::AsFeed for $site {
            fn as_feed(&self) -> miette::Result<Url> {
                use miette::IntoDiagnostic as _;
                format!($feed_template, &self.vanity)
                    .parse::<Url>()
                    .into_diagnostic()
            }
        }
    };
}

simple_account!(Cohost, "https://cohost.org/{}", "https://cohost.org/{}.rss");
simple_account!(
    DeviantArt,
    "https://deviantart.com/user/{}",
    "https://backend.deviantart.com/{}"
);
simple_account!(
    Gocomics,
    "https://gocomics.com/{}",
    "https://comicrss.com/rss/{}.rss"
);
simple_account!(Insta, "https://instagram.com/{}", "NA: {}");
simple_account!(
    Substack,
    "https://{}.substack.com/",
    "https://{}.substack.com/rss"
);
simple_account!(Tinyview, "https://tinyview.com/{}", "NA: {}");
simple_account!(
    Tumblr,
    "https://{}.tumblr.com/",
    "https://{}.tumblr.com/rss"
);

#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
// TODO: some hostname type
pub struct Bsky(#[knuffel(argument, str)] String);

#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Fedi {
    #[knuffel(argument, str)]
    instance: Url,

    #[knuffel(flatten(property))]
    account: Account,
}

#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
// TODO: channel ID & rss feed
pub struct Youtube(#[knuffel(argument)] String);

#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct HuginnId(#[knuffel(argument)] u16);

/// patreon profile, and monthly support amount
#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Patreon {
    #[knuffel(property, default)]
    per_month: u8,
    #[knuffel(flatten(property))]
    account: Account,
}
/// ko-fi profile, and monthly support amount
#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Kofi {
    #[knuffel(property, default)]
    per_month: u8,
    #[knuffel(flatten(property))]
    account: Account,
}
/// github profile, and monthly support amount
#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Github {
    #[knuffel(property, default)]
    per_month: u8,
    #[knuffel(flatten(property))]
    account: Account,
}

/**
/// name-or-id common pattern

 Most account handles can have
- (text) name
- (numeric) ID, eg snowflake

IDs are allocated once, and are innate.

DNS and bsky handles are hostnames, innate IDs.

TODO: hydrate name from (grand)parent node_name

TODO: these fields are `Option` because of `flatten`'s requirement.

*/
#[derive(Debug, Default, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Account {
    /// Mutable vain handle
    #[knuffel(property, default)]
    vanity: Option<Name>,
    /// Immutable humble, per-system internal identity
    #[knuffel(property)]
    serial: Option<Id>,
}

/// fb profile
#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Facebook {
    #[knuffel(flatten(property))]
    account: Account,
}

/// twt/X profile
#[derive(Debug, knuffel::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Twitter {
    #[knuffel(flatten(property))]
    account: Account,
    /// list memberships
    #[knuffel(child, default)]
    lists: Words,
}

mod display;
mod feed;
mod scoring;

#[cfg(test)]
mod tests {
    use super::*;
    use miette::IntoDiagnostic as _;

    #[test]
    fn aliases() -> miette::Result<()> {
        let input = r#"comic { aliases "artist1" "artist2";
                }"#;
        assert_eq!(
            vec![Art {
                handle: "comic".to_string(),
                aliases: Words(vec!["artist1".to_string(), "artist2".into()]),
                ..Default::default()
            }],
            knuffel::parse::<Vec<Art>>("", input)?
        );
        Ok(())
    }

    #[test]
    #[ignore = "try dropping /-"]
    fn default_handle() -> miette::Result<()> {
        let input = r#"comic { fb /-vanity="comic"
                }"#;
        assert_eq!(
            vec![Art {
                handle: "comic".to_string(),
                fb: Some(Facebook {
                    account: Account {
                        vanity: Some("comic".to_string().into()),
                        serial: None
                    }
                }),
                ..Default::default()
            }],
            knuffel::parse::<Vec<Art>>("", input)?
        );
        Ok(())
    }

    #[test]
    fn decode_fb() -> miette::Result<()> {
        let input = r#"
facebook vanity="abc"
facebook vanity=null serial="1234"
"#;
        assert_eq!(
            vec![
                Facebook {
                    account: Account {
                        vanity: Some("abc".to_string()),
                        serial: None
                    }
                },
                Facebook {
                    account: Account {
                        serial: Some(1234.to_string()),
                        vanity: None,
                    }
                }
            ],
            knuffel::parse::<Vec<Facebook>>("", input)?
        );
        Ok(())
    }

    #[test]
    fn where_am_i() -> miette::Result<()> {
        let input = r#"
hdhoang "https://hdhoang.space/" {
cohost "hdhoang"
    pat vanity="hdhoang"
    fb vanity="hdh0000"
 fedi "https://blob.cat/" vanity="hdhoang"
    twt vanity="21_25" serial="2125"
} "#;

        assert_eq!(
            knuffel::parse::<Vec<Art>>("", input)?,
            vec![Art {
                handle: "hdhoang".to_string(),
                web: Some(
                    "https://hdhoang.space/"
                        .parse::<url::Url>()
                        .into_diagnostic()?
                ),
                cohost: Cohost {
                    vanity: "hdhoang".to_string()
                }
                .into(),
                fedi: Fedi {
                    instance: "https://blob.cat".parse::<url::Url>().into_diagnostic()?,
                    account: Account {
                        vanity: Some("hdhoang".to_string()),
                        serial: None
                    },
                }
                .into(),
                pat: Some(Patreon {
                    per_month: 0,
                    account: Account {
                        vanity: Some("hdhoang".to_string()),
                        serial: None
                    }
                }),
                fb: Some(Facebook {
                    account: Account {
                        vanity: Some("hdh0000".to_string()),
                        serial: None
                    }
                }),
                twt: Some(Twitter {
                    account: Account {
                        vanity: Some("21_25".to_string()),
                        serial: Some(2125.to_string())
                    },
                    lists: Words(vec![]),
                }),
                ..Default::default()
            }]
        );
        Ok(())
    }
}
