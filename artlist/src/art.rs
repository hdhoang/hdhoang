#![allow(dead_code)]

use url::Url;

pub type Name = String;
pub type Id = String;

#[derive(Debug, Default, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Words(#[knus(arguments)] Vec<String>);

#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(Default, PartialEq))]
pub struct Art {
    /// prominent web handle
    #[knus(node_name)]
    handle: Name,
    #[knus(child, default)]
    aliases: Words,
    #[knus(child, default)]
    styles: Words,

    /// official website
    #[knus(argument, default, str)]
    web: Option<Url>,
    /// its feed, and readablility score
    #[knus(child)]
    feed: Option<feed::Feed>,

    /// linktr.ee, carrd.co, ... and the like
    #[knus(property, str)]
    link_list: Option<Url>,

    // capital pipes
    #[knus(child)]
    pat: Option<Patreon>,
    #[knus(child)]
    kofi: Option<Kofi>,
    #[knus(child)]
    github: Option<Github>,

    // 1-way feed
    #[knus(child)]
    tiny: Option<Tinyview>,
    #[knus(child)]
    fb: Option<Facebook>,
    #[knus(child)]
    go: Option<Gocomics>,
    #[knus(child)]
    tumblr: Option<Tumblr>,
    #[knus(child)]
    substack: Option<Substack>,
    #[knus(child)]
    bsky: Option<Bsky>,
    #[knus(child)]
    cohost: Option<Cohost>,
    #[knus(child)]
    da: Option<DeviantArt>,
    #[knus(child)]
    yt: Option<Youtube>,
    #[knus(child)]
    ig: Option<Insta>,

    /// Huginn agent id
    #[knus(child)]
    huginn: Option<HuginnId>,

    // interactive flows
    #[knus(child)]
    fedi: Option<Fedi>,
    #[knus(child)]
    twt: Option<Twitter>,
}

macro_rules! simple_account {
    ($site: ident, $url_template: expr, $feed_template: expr ) => {
        #[derive(Debug, knus::Decode)]
        #[cfg_attr(test, derive(PartialEq))]
        pub struct $site {
            #[knus(argument)]
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

#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
// TODO: some hostname type
pub struct Bsky(#[knus(argument, str)] String);

#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Fedi {
    #[knus(argument, str)]
    instance: Url,

    #[knus(flatten(property))]
    account: Account,
}

#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
// TODO: channel ID & rss feed
pub struct Youtube(#[knus(argument)] String);

#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct HuginnId(#[knus(argument)] u16);

/// patreon profile, and monthly support amount
#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Patreon {
    #[knus(property, default)]
    per_month: u8,
    #[knus(flatten(property))]
    account: Account,
}
/// ko-fi profile, and monthly support amount
#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Kofi {
    #[knus(property, default)]
    per_month: u8,
    #[knus(flatten(property))]
    account: Account,
}
/// github profile, and monthly support amount
#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Github {
    #[knus(property, default)]
    per_month: u8,
    #[knus(flatten(property))]
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
#[derive(Debug, Default, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Account {
    /// Mutable vain handle
    #[knus(property, default)]
    vanity: Option<Name>,
    /// Immutable humble, per-system internal identity
    #[knus(property)]
    serial: Option<Id>,
}

/// fb profile
#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Facebook {
    #[knus(flatten(property))]
    account: Account,
}

/// twt/X profile
#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Twitter {
    #[knus(flatten(property))]
    account: Account,
    /// list memberships
    #[knus(child, default)]
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
            knus::parse::<Vec<Art>>("", input)?
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
            knus::parse::<Vec<Art>>("", input)?
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
            knus::parse::<Vec<Facebook>>("", input)?
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
            knus::parse::<Vec<Art>>("", input)?,
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
