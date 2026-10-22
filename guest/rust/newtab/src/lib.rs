//! Rapidkoi's new tab (PRP-027, PRP-030): the app every empty tab runs,
//! built into `./main`. It shows the wordmark, where you left off with
//! Continue, and your pins, asking Rapidkoi for them each time it's opened
//! or shown again. Its manifest declares the built-in permissions
//! `read_pins`, `read_left_off` and `open_apps` (`scripts/newtab.sh`).
//! Text and buttons only until contract 3 can draw images (step B).
//!
//! Build and pack: `scripts/newtab.sh`, which writes `assets/apps/newtab.rkb`.

use minicbor::{Decode, Encode};
use rapidkoi::{button, column, row, text, App, Ask, Event, Place, Space, View};

/// What the new tab shows of one app.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Shown {
    #[n(0)]
    name: String,
    #[n(1)]
    place: String,
    #[n(2)]
    title: Option<String>,
}

impl From<Place> for Shown {
    fn from(p: Place) -> Shown {
        Shown { name: p.name, place: p.place, title: p.title }
    }
}

#[derive(Default, Encode, Decode)]
struct NewTab {
    /// Your pins, as last read.
    #[n(0)]
    pins: Vec<Shown>,
    /// Where you left off, as last read.
    #[n(1)]
    left: Option<Shown>,
}

/// Each time it's opened or back at the front: what may have changed.
fn read() -> Vec<Ask> {
    vec![Ask::pins("pins"), Ask::left_off("left")]
}

impl App for NewTab {
    fn handle(self, event: Event) -> (Self, Vec<Ask>) {
        match event {
            Event::Opened | Event::Resume => (self, read()),
            Event::Data { msg, cbor } if msg == "pins" => (
                NewTab { pins: Place::list(&cbor).unwrap_or_default().into_iter().map(Shown::from).collect(), ..self },
                vec![],
            ),
            Event::Data { msg, cbor } if msg == "left" => {
                (NewTab { left: Place::one(&cbor).map(Shown::from), ..self }, vec![])
            }
            // Nothing left off.
            Event::Answer { msg, .. } if msg == "left" => (NewTab { left: None, ..self }, vec![]),
            Event::Message { msg, .. } if msg == "continue" => (self, vec![Ask::continue_left_off("continue")]),
            Event::Message { msg, .. } => match msg.strip_prefix("pin:").and_then(|i| i.parse::<u32>().ok()) {
                Some(i) => (self, vec![Ask::open_pin("open", i)]),
                None => (self, vec![]),
            },
            _ => (self, vec![]),
        }
    }

    fn view(&self) -> View {
        let mut parts = vec![text("Rapidkoi").title()];
        if let Some(l) = &self.left {
            parts.push(
                column([
                    text("Where you left off").heading(),
                    text(&l.name),
                    text(l.title.as_deref().unwrap_or(&l.place)).dim(),
                    button("Continue").primary().on_press("continue"),
                ])
                .spacing(Space::Sm),
            );
        }
        if !self.pins.is_empty() {
            let buttons = self.pins.iter().enumerate().map(|(i, p)| button(&p.name).on_press(&format!("pin:{}", i)));
            parts.push(column([text("Pinned").heading(), row(buttons).spacing(Space::Sm)]).spacing(Space::Sm));
        }
        column(parts).spacing(Space::Lg)
    }
}

rapidkoi::export!(NewTab);

#[cfg(test)]
mod tests {
    use super::*;

    fn shown(name: &str) -> Shown {
        Shown { name: name.into(), place: format!("{}.example", name), title: None }
    }

    fn message(msg: &str) -> Vec<Ask> {
        NewTab::default().handle(Event::Message { msg: msg.into(), value: None }).1
    }

    #[test]
    fn it_reads_on_open_and_on_resume() {
        assert_eq!(NewTab::default().handle(Event::Opened).1, read());
        assert_eq!(NewTab::default().handle(Event::Resume).1, read());
    }

    #[test]
    fn with_nothing_it_shows_the_wordmark() {
        assert_eq!(NewTab::default().view(), column([text("Rapidkoi").title()]).spacing(Space::Lg));
    }

    #[test]
    fn continue_shows_only_with_somewhere_to_continue() {
        let t = NewTab { left: Some(shown("Quote")), pins: vec![] };
        assert_eq!(
            t.view(),
            column([
                text("Rapidkoi").title(),
                column([
                    text("Where you left off").heading(),
                    text("Quote"),
                    text("Quote.example").dim(),
                    button("Continue").primary().on_press("continue"),
                ])
                .spacing(Space::Sm),
            ])
            .spacing(Space::Lg)
        );
        assert_eq!(message("continue"), vec![Ask::continue_left_off("continue")]);
    }

    #[test]
    fn each_pin_opens_by_its_place_in_the_list() {
        let t = NewTab { left: None, pins: vec![shown("Quote"), shown("Counter")] };
        assert_eq!(
            t.view(),
            column([
                text("Rapidkoi").title(),
                column([
                    text("Pinned").heading(),
                    row([button("Quote").on_press("pin:0"), button("Counter").on_press("pin:1")]).spacing(Space::Sm),
                ])
                .spacing(Space::Sm),
            ])
            .spacing(Space::Lg)
        );
        assert_eq!(message("pin:1"), vec![Ask::open_pin("open", 1)]);
        assert_eq!(message("pin:x"), vec![]);
    }

    #[test]
    fn nothing_left_off_clears_it() {
        let t = NewTab { left: Some(shown("Quote")), pins: vec![] };
        let (t, _) = t.handle(Event::Answer { msg: "left".into(), value: None });
        assert_eq!(t.left, None);
    }
}
