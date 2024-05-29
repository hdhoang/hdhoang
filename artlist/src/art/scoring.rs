use crate::*;

/// Value-judgment on each feed quality
pub trait Readable {
    fn score(&self) -> u8 {
        0
    }
}

impl Readable for Art {
    fn score(&self) -> u8 {
        self.feed.as_ref().map(|f| f.score()).unwrap_or_default()
    }
}

impl Readable for Patreon {
    /// Needs conversion from mails. Shows only 1 image.
    fn score(&self) -> u8 {
        3
    }
}
impl Readable for Kofi {
    /// Needs conversion from mails
    fn score(&self) -> u8 {
        3
    }
}
impl Readable for Facebook {
    /// Limited to 30 feeds, supplied by Inoreader
    fn score(&self) -> u8 {
        5
    }
}

impl Readable for Insta {
    /// Needs conversion via addons, and polling
    fn score(&self) -> u8 {
        1
    }
}

impl Readable for DeviantArt {
    /// Full image (so far)
    fn score(&self) -> u8 {
        9
    }
}
impl Readable for Tumblr {
    /// Full images. Nested quoting becomes unreadable.
    fn score(&self) -> u8 {
        8
    }
}
