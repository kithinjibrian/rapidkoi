//! Rapidkoi's new tab (PRP-027, PRP-030): the app every empty tab runs,
//! built into `./main`. It shows the wordmark, where you left off with
//! Continue, and your pins, asking Rapidkoi for them each time it's opened,
//! shown again, or told they changed (PRP-031). Its manifest declares the built-in permissions
//! `read_pins`, `read_left_off` and `open_apps` (`scripts/newtab.sh`).
//! Text and buttons only until contract 3 can draw images (step B).
//!
//! Build and pack: `scripts/newtab.sh`, which writes `assets/apps/newtab.rkb`.

use minicbor::{Decode, Encode};
use rapidkoi::{button, column, row, text, App, Ask, Event, Next, Place, Session, Space, State, View};

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

#[derive(Default, State)]
struct NewTab {
    /// Your pins, as last read.
    pins: Vec<Shown>,
    /// Where you left off, as last read.
    left: Option<Shown>,
    /// How many windows the last session had, if there's one to restore (PRP-035).
    session: Option<u32>,
}

/// Each time it's opened or back at the front: what may have changed.
fn read() -> Vec<Ask> {
    vec![Ask::pins("pins"), Ask::left_off("left"), Ask::session("session")]
}

impl App for NewTab {
    fn handle(self, event: Event) -> Next<Self> {
        // The new tab has no address of its own to put fields in.
        self.step(event).into()
    }

    fn view(&self) -> View {
        self.screen()
    }
}

impl NewTab {
    fn step(self, event: Event) -> (Self, Vec<Ask>) {
        match event {
            Event::Opened | Event::Resume | Event::Change => (self, read()),
            Event::Data { msg, cbor } if msg == "pins" => (
                NewTab { pins: Place::list(&cbor).unwrap_or_default().into_iter().map(Shown::from).collect(), ..self },
                vec![],
            ),
            Event::Data { msg, cbor } if msg == "left" => {
                (NewTab { left: Place::one(&cbor).map(Shown::from), ..self }, vec![])
            }
            Event::Data { msg, cbor } if msg == "session" => {
                (NewTab { session: Session::read(&cbor).map(|s| s.count), ..self }, vec![])
            }
            // Nothing left off, or no session to restore.
            Event::Answer { msg, .. } if msg == "left" => (NewTab { left: None, ..self }, vec![]),
            Event::Answer { msg, .. } if msg == "session" => (NewTab { session: None, ..self }, vec![]),
            Event::Message { msg, .. } if msg == "restore" => (self, vec![Ask::restore_session("restore")]),
            Event::Message { msg, .. } if msg == "continue" => (self, vec![Ask::continue_left_off("continue")]),
            Event::Message { msg, .. } => match msg.strip_prefix("pin:").and_then(|i| i.parse::<u32>().ok()) {
                Some(i) => (self, vec![Ask::open_pin("open", i)]),
                None => (self, vec![]),
            },
            _ => (self, vec![]),
        }
    }

    fn screen(&self) -> View {
        let mut parts = vec![text("Rapidkoi").title()];
        if let Some(n) = self.session {
            parts.push(button(&format!("Restore previous session ({})", n)).primary().on_press("restore"));
        }
        if let Some(l) = &self.left {
            parts.push(
                column([
                    text("Where you left off").heading(),
                    text(&l.name),
                    // Its screen's title, unless that only repeats its name: then where it lives.
                    text(match l.title.as_deref() {
                        Some(t) if t != l.name => t,
                        _ => &l.place,
                    })
                    .dim(),
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
        NewTab::default().step(Event::Message { msg: msg.into(), value: None }).1
    }

    #[test]
    fn it_reads_on_open_resume_and_change() {
        assert_eq!(NewTab::default().step(Event::Opened).1, read());
        assert_eq!(NewTab::default().step(Event::Resume).1, read());
        assert_eq!(NewTab::default().step(Event::Change).1, read());
    }

    #[test]
    fn with_nothing_it_shows_the_wordmark() {
        assert_eq!(NewTab::default().view(), column([text("Rapidkoi").title()]).spacing(Space::Lg));
    }

    #[test]
    fn continue_shows_only_with_somewhere_to_continue() {
        let t = NewTab { left: Some(shown("Quote")), pins: vec![], session: None };
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
        let t = NewTab { left: None, pins: vec![shown("Quote"), shown("Counter")], session: None };
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
    fn restore_shows_only_with_a_session() {
        // PRP-035.
        let t = NewTab { session: Some(2), ..NewTab::default() };
        assert_eq!(
            t.view(),
            column([text("Rapidkoi").title(), button("Restore previous session (2)").primary().on_press("restore")])
                .spacing(Space::Lg)
        );
        assert_eq!(message("restore"), vec![Ask::restore_session("restore")]);
        let (t, _) = t.step(Event::Answer { msg: "session".into(), value: None });
        assert_eq!(t.session, None);
    }

    #[test]
    fn nothing_left_off_clears_it() {
        let t = NewTab { left: Some(shown("Quote")), pins: vec![], session: None };
        let (t, _) = t.step(Event::Answer { msg: "left".into(), value: None });
        assert_eq!(t.left, None);
    }
}
