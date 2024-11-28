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

impl fmt::Display for Patreon {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(
            f,
            "https://patreon.com/c/{}/about",
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
        if let Some(ref handle) = self.account.vanity {
            write!(f, "https://bsky.app/profile/{handle}")?;
            if !handle.contains('.') {
                write!(f, ".bsky.social")?
            }
        } else {
            let did = self
                .account
                .serial
                .clone()
                .unwrap_or_else(|| "handle.invalid".into());
            write!(f, "https://bsky.app/profile/did:plc:{did}")?
        }
        Ok(())
    }
}
