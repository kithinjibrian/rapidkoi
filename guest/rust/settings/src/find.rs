//! Settings' categories (PRP-075) and Find: each category's name, icon and
//! the words Find matches it by.

/// One of Settings' categories, in the order the side lists them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Appearance,
    Startup,
    You,
    Publishers,
    About,
}

/// Every category, in order.
pub const PAGES: [Page; 5] = [Page::Appearance, Page::Startup, Page::You, Page::Publishers, Page::About];

impl Page {
    /// Its place in [`PAGES`]: what the state keeps and messages carry.
    pub fn index(self) -> u8 {
        PAGES.iter().position(|p| *p == self).unwrap_or(0) as u8
    }

    /// The category at `i`; the first for anything else.
    pub fn at(i: u8) -> Page {
        PAGES.get(i as usize).copied().unwrap_or(Page::Appearance)
    }

    /// Its name, in the side and as the page's title.
    pub fn name(self) -> &'static str {
        match self {
            Page::Appearance => "Appearance",
            Page::Startup => "Startup",
            Page::You => "You",
            Page::Publishers => "Publishers",
            Page::About => "About Rapidkoi",
        }
    }

    /// Its icon, one of the bundle's assets.
    pub fn icon(self) -> &'static str {
        match self {
            Page::Appearance => "brush.svg",
            Page::Startup => "startup.svg",
            Page::You => "user.svg",
            Page::Publishers => "shield.svg",
            Page::About => "info.svg",
        }
    }

    /// Whether a line goes above it in the side: the three groups.
    pub fn starts_group(self) -> bool {
        matches!(self, Page::You | Page::About)
    }

    /// What Find matches it by, besides its name.
    fn words(self) -> &'static [&'static str] {
        match self {
            Page::Appearance => &["theme", "dark", "light", "contrast", "text size", "reading"],
            Page::Startup => &["open", "restore", "session", "new tab", "start"],
            Page::You => &["name", "sign in", "signed in", "key", "account"],
            Page::Publishers => &["keys", "trust", "signed", "fingerprint"],
            Page::About => &["version", "contract"],
        }
    }

    /// Whether Find's text (any case) matches it; empty matches every one.
    pub fn matches(self, typed: &str) -> bool {
        let q = typed.trim().to_lowercase();
        q.is_empty() || self.name().to_lowercase().contains(&q) || self.words().iter().any(|w| w.contains(&q))
    }
}

/// The categories Find's text matches, in order.
pub fn found(typed: &str) -> Vec<Page> {
    PAGES.iter().copied().filter(|p| p.matches(typed)).collect()
}
