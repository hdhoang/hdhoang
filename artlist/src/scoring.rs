use crate::*;

/// Value-judgment on each feed quality
pub trait Readable {
    fn score(&self) -> u8 {
        0
    }
}

impl Readable for Art {
    fn score(&self) -> u8 {
        self.feed.readable
    }
}

impl Readable for Patreon {
    /// Needs inoreader's mail conversion
    fn score(&self) -> u8 {
        3
    }
}
impl Readable for Kofi {
    /// Needs inoreader's mail conversion
    fn score(&self) -> u8 {
        3
    }
}

impl Readable for Insta {
    /// TDMA Needs nitter's conversion via multiple instances
    fn score(&self) -> u8 {
        2
    }
}

impl Readable for Insta {
    /// TDMA Needs granary's conversion via addons, and polling
    fn score(&self) -> u8 {
        1
    }
}

impl Readable for DeviantArt {
    /// Full image
    fn score(&self) -> u8 {
        9
    }
}
impl Readable for Tumblr {
    /// Full images
    fn score(&self) -> u8 {
        8
    }
}
