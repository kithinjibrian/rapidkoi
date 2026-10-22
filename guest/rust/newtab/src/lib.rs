//! Rapidkoi's new tab (PRP-027): the app every empty tab runs, built into
//! `./main`. For now it shows only the wordmark; a logo waits for images in
//! the contract.
//!
//! Build and pack: `scripts/newtab.sh`, which writes `assets/apps/newtab.rkb`.

use minicbor::{Decode, Encode};
use rapidkoi::{column, text, App, Ask, Event, View};

/// No state: the page is the same every time.
#[derive(Default, Encode, Decode)]
struct NewTab {}

impl App for NewTab {
    fn handle(self, _event: Event) -> (Self, Vec<Ask>) {
        (self, vec![])
    }

    fn view(&self) -> View {
        column([text("Rapidkoi").title()])
    }
}

rapidkoi::export!(NewTab);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_shows_the_wordmark() {
        assert_eq!(NewTab::default().view(), column([text("Rapidkoi").title()]));
    }

    #[test]
    fn it_asks_for_nothing() {
        let (_, asks) = NewTab::default().handle(Event::Opened);
        assert_eq!(asks, vec![]);
        let (_, asks) = NewTab::default().handle(Event::Message { msg: "any".into(), value: None });
        assert_eq!(asks, vec![]);
    }
}
