use std::fmt;

use crate::*;
impl fmt::Display for Art {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "{}", self.handle)
    }
}

impl fmt::Display for Facebook {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(
            f,
            "https://facebook.com/{}",
            self.account.vanity.clone().unwrap_or_default()
        )
    }
}

impl fmt::Display for HuginnId {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "https://fly.dev/agents/{}", self.0)
    }
}

impl fmt::Display for Bsky {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let slug = self.account.vanity.clone().unwrap_or_default();
        write!(f, "https://bsky.app/profile/{slug}")?;
        if !slug.contains('.') {
            write!(f, ".bsky.social")?
        }
        Ok(())
    }
}
