use miette::IntoDiagnostic as _;
use url::Url;

use super::*;

/// First-party syndication feed
#[derive(Debug, knus::Decode)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Feed {
    #[knus(argument, str)]
    url: Url,
    #[knus(property, default)]
    readable: u8,
    #[knus(property)]
    /// justification for the readable score
    reason: Option<String>,
}

impl Feed {
    pub fn score(&self) -> u8 {
        self.readable
    }
}

pub trait AsFeed {
    fn as_feed(&self) -> miette::Result<Url>;
}

impl AsFeed for Art {
    fn as_feed(&self) -> miette::Result<Url> {
        self.feed
            .as_ref()
            .map(|f| f.url.clone())
            .ok_or_else(|| todo!("missing native feed"))
    }
}

impl AsFeed for Facebook {
    fn as_feed(&self) -> miette::Result<Url> {
        self.to_string().parse().into_diagnostic()
    }
}
