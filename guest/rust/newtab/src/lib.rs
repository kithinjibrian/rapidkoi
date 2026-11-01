//! Rapidkoi's new tab (PRP-027): the app every empty tab runs, built into
//! `./main`. A copy of its target design (`docs/designs/newtab-target.md`,
//! board A5), becoming real a part at a time. Real: Restore, which asks for
//! your last session each time the tab is shown and shows its apps' icons
//! (PRP-060: `icon:PLACE`, drawn by Rapidkoi). Still placeholders, doing
//! nothing: notifications, Recent and Pinned.
//!
//! Drawn with contract 10's looks (PRP-059): gradients for the washes, the
//! page's glow and Rapidkoi's mark; layers for a face or an icon with
//! something on its corner, and for Restore's overlapping icons; wraps for
//! the two-column sections, which fall to one column on a narrow window;
//! a fitting row for Pinned. What stays a picture in `assets/` (packed by
//! `scripts/newtab.sh`): the wordmark (GTK can't fill text with a
//! gradient), the beach (the picture a real app would give its place), and
//! the apps' icons.

use rapidkoi::raw::boxed;
use rapidkoi::style::{
    Align, Background, Border, Color, Corner, Font, Layout, Place, Sizing, Spacing, State as Look, Style, Text,
    TextSize, Weight, When,
};
use rapidkoi::{
    column, fit, image, layers, row, spacer, text, wrap, App, Ask, Event, Fit, Message, Next, Session, Space, State,
    View,
};

/// Every picture the screen shows, each packed from `assets/` under its name.
const ASSETS: [&str; 7] =
    ["wordmark.svg", "beach.svg", "notes.svg", "counter.svg", "trip.svg", "settings.svg", "board.svg"];

/// The card colour behind a badge's ring, so the ring reads as a gap.
const CARD: (u8, u8, u8) = (0x1e, 0x1e, 0x1e);
/// The Restore pill's colour behind its icons' rings.
const PILL: (u8, u8, u8) = (0x23, 0x23, 0x23);

/// What the new tab's screen sends.
#[derive(Debug, Clone, PartialEq, Message)]
enum Msg {
    /// Bring back the last session.
    Restore,
}

type V = View<Msg>;

#[derive(Default, State)]
struct NewTab {
    /// The last session's apps' icons, in order (`icon:PLACE`), while there's one to restore.
    restore: Option<Vec<String>>,
}

impl App for NewTab {
    type Msg = Msg;

    fn handle(self, event: Event<Msg>) -> Next<Self> {
        // The new tab has no address of its own to put fields in.
        self.step(event).into()
    }

    fn view(&self) -> V {
        // The page fills the window's width (the owner's choice over the design's cap).
        let page = column([top(&self.restore), notifications(), recent(), pinned()])
            .style([Spacing::padding_xy(Space::Px(32), Space::Px(36)), Spacing::gap(Space::Px(32))]);
        // Rapidkoi never scrolls an app's page (PRP-046): the page scrolls itself,
        // under a faint glow of the brand's colours from its top-left corner.
        column([page]).style([
            Layout::scroll(),
            Layout::grow(),
            Background::ellipse(
                0.0,
                0.0,
                0.6,
                0.35,
                [
                    (Color::rgba(255, 119, 0, 31), 0.0),
                    (Color::rgba(255, 46, 99, 10), 0.55),
                    (Color::rgba(255, 46, 99, 0), 0.8),
                ],
            ),
        ])
    }
}

rapidkoi::export!(NewTab);

impl NewTab {
    fn step(self, event: Event<Msg>) -> (Self, Vec<Ask>) {
        match event {
            // Each time it's shown: the session may have changed.
            Event::Opened | Event::Resume => (self, vec![Ask::session("session")]),
            Event::Data { msg, cbor } if msg == "session" => {
                let icons = Session::read(&cbor).map(|s| s.apps.into_iter().map(|p| p.icon).collect());
                (NewTab { restore: icons, ..self }, vec![])
            }
            // No session to restore.
            Event::Answer { msg, .. } if msg == "session" => (NewTab { restore: None, ..self }, vec![]),
            Event::Message { msg: Msg::Restore, .. } => (self, vec![Ask::restore_session("restore")]),
            _ => (self, vec![]),
        }
    }
}

/// The icons Restore shows, at most three, and how many apps more.
fn shown_and_more(icons: &[String]) -> (&[String], usize) {
    let n = icons.len().min(3);
    (&icons[..n], icons.len() - n)
}

/// A picture from the bundle, at its size.
fn picture(name: &str, w: u16, h: u16) -> V {
    debug_assert!(ASSETS.contains(&name), "{name} isn't packed");
    image(&format!("asset:{name}")).style([Sizing::width(w), Sizing::height(h)])
}

fn rgb(c: (u8, u8, u8)) -> Color {
    Color::rgb(c.0, c.1, c.2)
}

/// The page's colour a step towards the text: cards, pills, hovers.
fn lift(by: f64) -> Color {
    Color::system("background").mix(Color::system("text"), by)
}

/// A section's name: small, bold and dim.
fn heading(s: &str) -> V {
    text(s).style([Font::size(TextSize::Small), Font::weight(Weight::Bold), Text::color(Color::system("text_dim"))])
}

/// Small, dim text.
fn note(s: &str) -> V {
    text(s).style([Font::size(TextSize::Small), Text::color(Color::system("text_dim"))])
}

/// A round button's look: the accent for the one to press first, else the surface.
fn pill(word: &str, first: bool) -> V {
    let (fill, ink) = if first { ("accent", "on_accent") } else { ("surface", "text") };
    let weight = if first { Weight::Bold } else { Weight::Normal };
    boxed()
        .style([
            Background::color(Color::system(fill)),
            Border::corner(Corner::Round),
            Spacing::padding_xy(Space::Px(6), Space::Px(14)),
            Layout::align(Align::Center),
        ])
        .children([text(word).style([
            Text::color(Color::system(ink)),
            Font::size(TextSize::Small),
            Font::weight(weight),
        ])])
}

/// A card's frame: a lifted fill, a thin edge, round corners.
fn card(round: u16, hover_edge: bool) -> Vec<Style> {
    let hover = if hover_edge {
        When(Look::Hover, [Border::all(1, Color::system("accent"))])
    } else {
        When(Look::Hover, [Background::color(lift(0.07))])
    };
    vec![
        Background::color(lift(0.04)),
        Border::all(1, Color::system("surface")),
        Border::corner(Corner::Px(round)),
        hover,
    ]
}

/// A word centred in a round box of `size` pixels: an initial, a mark.
fn disc(word: &str, size: u16, fill: Vec<Style>, ink: Color, words: TextSize, ring: Option<Color>) -> V {
    let mut look =
        vec![Sizing::width(size), Sizing::height(size), Border::corner(Corner::Round), Layout::align(Align::Center)];
    if let Some(c) = ring {
        look.push(Border::all(2, c));
    }
    look.extend(fill);
    column([
        spacer(),
        text(word).style([
            Text::color(ink),
            Font::size(words),
            Font::weight(Weight::Bold),
            Layout::align(Align::Center),
        ]),
        spacer(),
    ])
    .style(look)
}

/// An app's icon with a ring of the colour behind it, so neighbours or a base show a gap.
fn ringed(icon: &str, size: u16, behind: Color) -> V {
    picture(icon, size, size).style([Border::all(2, behind), Border::corner(Corner::Px(size * 2 / 7))])
}

/// Something 44 pixels square with a small thing on its bottom-end corner.
fn cornered(base: V, badge: V) -> V {
    // The base sits at the top-start of a 44-pixel square, so the badge overhangs it as drawn.
    let square = column([base]).style([Sizing::width(44), Sizing::height(44)]);
    layers([square, badge.style([Layout::place(Place::BottomEnd)])])
}

/// A person's face (their initials on their colour) with their app on its corner.
fn face(initials: &str, colour: (u8, u8, u8), app: &str) -> V {
    let circle =
        disc(initials, 40, vec![Background::color(rgb(colour))], Color::rgb(255, 255, 255), TextSize::Small, None);
    cornered(circle.style([Layout::align(Align::Start)]), ringed(app, 18, rgb(CARD)))
}

/// Rapidkoi's mark: an "R" on the brand's gradient.
fn mark(size: u16) -> V {
    let brand = Background::linear(135, [(Color::rgb(0xff, 0x77, 0x00), 0.0), (Color::rgb(0xff, 0x2e, 0x63), 1.0)]);
    disc("R", size, vec![brand], Color::rgb(0x16, 0x16, 0x16), TextSize::Rem(0.6), Some(rgb(CARD)))
}

/// The wordmark, and, while there's a session to restore, Restore with the apps it brings back.
fn top(restore: &Option<Vec<String>>) -> V {
    let wordmark = picture("wordmark.svg", 176, 44).label("Rapidkoi");
    let Some(icons) = restore else {
        return row([wordmark]);
    };
    let (shown, more) = shown_and_more(icons);
    // Each overlaps the one before, by its place in a box narrower than them all.
    let width = 26 + 18 * (shown.len().max(1) as u16 - 1);
    let mut apps = vec![boxed().style([Sizing::width(width), Sizing::height(26)])];
    for (i, icon) in shown.iter().enumerate() {
        let place = match (i, shown.len()) {
            (0, _) => Place::Start,
            (i, n) if i + 1 == n => Place::End,
            _ => Place::Center,
        };
        apps.push(image(icon).style([
            Sizing::width(26),
            Sizing::height(26),
            Border::all(2, rgb(PILL)),
            Border::corner(Corner::Px(8)),
            Layout::place(place),
        ]));
    }
    let mut parts = vec![
        // The pill's text sits further from its edge than Restore does.
        boxed().style([Sizing::width(8)]),
        text("Restore previous session").style([Layout::align(Align::Center)]),
        layers(apps).style([Layout::align(Align::Center)]),
    ];
    if more > 0 {
        parts.push(note(&format!("+{more}")).style([Layout::align(Align::Center)]));
    }
    parts.push(pill("Restore", true).on_press(Msg::Restore));
    let pill_row = row(parts).style([
        Spacing::gap(Space::Md),
        Spacing::padding(Space::Px(6)),
        Background::color(lift(0.06)),
        Border::all(1, Color::system("border")),
        Border::corner(Corner::Round),
        Layout::align(Align::Center),
    ]);
    row([wordmark, spacer(), pill_row])
}

/// One notification: who or what it's from, its words (each bold or not), and its button if it has one.
fn notice(from: V, words: &[(&str, bool)], detail: &str, button: Option<(&str, bool)>) -> V {
    let line = row(words.iter().map(|(w, bold)| {
        let t = text(w);
        if *bold {
            t.style([Font::weight(Weight::Bold)])
        } else {
            t
        }
    }));
    let mut parts = vec![
        from.style([Layout::align(Align::Center)]),
        column([line, note(detail)]).style([Spacing::gap(Space::Px(3)), Layout::grow(), Layout::align(Align::Center)]),
    ];
    if let Some((word, first)) = button {
        parts.push(pill(word, first));
    }
    row(parts).style(
        [vec![Spacing::gap(Space::Px(14)), Spacing::padding_xy(Space::Px(12), Space::Px(14))], card(14, false)]
            .concat(),
    )
}

/// Two equal columns that fall to one on a narrow window.
fn halves(children: impl IntoIterator<Item = V>) -> V {
    wrap(children).style([Layout::equal(), Layout::per_line(2), Spacing::gap(Space::Px(16))])
}

/// Notifications: the count, See all, then the newest four, two by two.
fn notifications() -> V {
    let count = boxed()
        .style([
            Background::color(Color::system("accent")),
            Border::corner(Corner::Round),
            Spacing::padding_xy(Space::None, Space::Px(7)),
        ])
        .children([text("4").style([
            Text::color(Color::system("on_accent")),
            Font::size(TextSize::Small),
            Font::weight(Weight::Bold),
        ])]);
    let head = row([heading("NOTIFICATIONS").style([Layout::align(Align::Center)]), count, spacer(), note("See all")])
        .style([Spacing::gap(Space::Sm)]);
    let invite = notice(
        face("AO", (0xb4, 0x53, 0x09), "board.svg"),
        &[("Amina invited you to ", false), ("Sprint 3", true)],
        "Board · live now",
        Some(("Join", true)),
    );
    let shared = notice(
        face("WK", (0x0e, 0x74, 0x90), "trip.svg"),
        &[("Wanjiku shared ", false), ("Mombasa, 12–15 Nov", true)],
        "Trip · 20 min ago",
        Some(("Open", false)),
    );
    let update = notice(
        cornered(picture("notes.svg", 40, 40).style([Layout::align(Align::Start)]), mark(18)),
        &[("Notes 2.5", true), (" is ready", false)],
        "notes.dafifi.net · 1 h ago",
        Some(("Update", false)),
    );
    let own = notice(picture("notes.svg", 40, 40), &[("“Shopping list” is due today", false)], "Notes · 2 h ago", None);
    column([head, halves([invite, shared, update, own])]).style([Spacing::gap(Space::Px(12))])
}

/// A small card's picture area without an app's picture: its colour washing
/// in from the top-left, and its icon on it.
fn wash(icon: &str, colour: (u8, u8, u8)) -> V {
    let (r, g, b) = colour;
    column([picture(icon, 44, 44).style([Layout::align(Align::Start)])]).style([
        Sizing::height(84),
        Spacing::padding(Space::Px(12)),
        Border::corner(Corner::Px(8)),
        Background::color(Color::rgb(0x1b, 0x1b, 0x1b)),
        Background::ellipse(
            0.0,
            0.0,
            1.1,
            1.5,
            [(Color::rgba(r, g, b, 128), 0.0), (Color::rgba(r, g, b, 31), 0.6), (Color::rgba(r, g, b, 0), 0.9)],
        ),
    ])
}

/// A small Recent card: its picture area, title and details.
fn mini(icon: &str, colour: (u8, u8, u8), title: &str, detail: &str) -> V {
    column([
        wash(icon, colour),
        column([text(title).style([Font::weight(Weight::Bold), Text::truncate()]), note(detail)])
            .style([Spacing::padding_xy(Space::None, Space::Px(4)), Spacing::gap(Space::Px(3))]),
    ])
    .style([vec![Spacing::padding(Space::Px(8)), Spacing::gap(Space::Px(10))], card(14, true)].concat())
}

/// Recent: the newest place large in the left half, the next four small in the right.
fn recent() -> V {
    let large = column([
        // As wide as the card, cut to its height (`beach.svg` is drawn wide for this).
        image("asset:beach.svg").fit(Fit::Cover).style([Sizing::height(240), Border::corner(Corner::Px(10))]),
        column([
            text("Mombasa, 12–15 Nov").style([Font::size(TextSize::Rem(1.45)), Font::weight(Weight::Bold)]),
            text("3 nights · 2 bookings confirmed"),
            row([picture("trip.svg", 18, 18), note("Trip · localhost:8000/trips/12 · 2 min ago")])
                .style([Spacing::gap(Space::Sm)]),
        ])
        .style([Spacing::padding_xy(Space::None, Space::Px(8)), Spacing::gap(Space::Px(6))]),
    ])
    .style([vec![Spacing::padding(Space::Px(12)), Spacing::gap(Space::Px(14))], card(16, true)].concat());
    let teal = (20, 184, 166);
    let small = halves([
        mini("notes.svg", teal, "Shopping list", "4 items left · Notes · 1 h ago"),
        mini("counter.svg", (139, 92, 246), "Counter", "Counter · Yesterday"),
        mini("notes.svg", teal, "Ideas for the new tab", "Notes · Yesterday"),
        mini("settings.svg", (113, 113, 122), "Appearance", "Settings · Monday"),
    ]);
    column([heading("RECENT"), halves([large, small])]).style([Spacing::gap(Space::Px(12))])
}

/// Pinned: as many apps' icons and names as fit on a line, and See all.
fn pinned() -> V {
    let tile = |icon: &str, name: &str| {
        column([
            picture(icon, 56, 56).style([Layout::align(Align::Center)]),
            text(name).style([Font::size(TextSize::Small), Layout::align(Align::Center)]),
        ])
        .style([Sizing::width(80), Spacing::gap(Space::Sm)])
    };
    // One line of pins, then See all (the owner's choice): the line ends
    // cleanly at the window's edge, the rest hidden.
    column([
        row([heading("PINNED"), spacer(), note("See all")]),
        fit([
            tile("notes.svg", "Notes"),
            tile("counter.svg", "Counter"),
            tile("trip.svg", "Trip"),
            tile("settings.svg", "Settings"),
        ])
        .style([Spacing::gap(Space::Lg)]),
    ])
    .style([Spacing::gap(Space::Px(12))])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_screen_names_only_packed_pictures() {
        // `picture` checks each name against ASSETS while the view is built.
        let _ = NewTab::default().view();
    }

    #[test]
    fn every_picture_is_in_assets_and_packed() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/assets/");
        let script = include_str!("../../../../scripts/newtab.sh");
        for name in ASSETS {
            assert!(std::path::Path::new(&format!("{dir}{name}")).is_file(), "assets/{name} is missing");
            assert!(script.contains(&format!("--asset {name}=")), "scripts/newtab.sh doesn't pack {name}");
        }
    }

    use minicbor::Encoder;

    /// A session answer: `count` windows of these apps, each `{name, place, icon}`.
    fn session(count: u32, places: &[&str]) -> Vec<u8> {
        let mut e = Encoder::new(Vec::new());
        e.map(2).unwrap().str("count").unwrap().u32(count).unwrap();
        e.str("apps").unwrap().array(places.len() as u64).unwrap();
        for p in places {
            e.map(3).unwrap();
            e.str("name").unwrap().str(p).unwrap();
            e.str("place").unwrap().str(p).unwrap();
            e.str("icon").unwrap().str(&format!("icon:{p}")).unwrap();
        }
        e.into_writer()
    }

    #[test]
    fn it_asks_for_the_last_session_each_time_it_s_shown() {
        for e in [Event::Opened, Event::Resume] {
            assert_eq!(NewTab::default().step(e).1, vec![Ask::session("session")]);
        }
    }

    #[test]
    fn restore_shows_the_session_s_apps_while_there_is_one() {
        let data = Event::Data { msg: "session".into(), cbor: session(5, &["notes.dafifi.net", "trip.app"]) };
        let (t, asks) = NewTab::default().step(data);
        assert_eq!(t.restore, Some(vec!["icon:notes.dafifi.net".to_string(), "icon:trip.app".to_string()]));
        assert!(asks.is_empty());
        let _ = t.view();
        // Nothing to restore: no pill.
        let (none, _) = t.step(Event::Answer { msg: "session".into(), value: None });
        assert_eq!(none.restore, None);
    }

    #[test]
    fn at_most_three_icons_then_how_many_more() {
        let icons: Vec<String> = (0..5).map(|i| format!("icon:a{i}")).collect();
        assert_eq!(shown_and_more(&icons), (&icons[..3], 2));
        assert_eq!(shown_and_more(&icons[..2]), (&icons[..2], 0));
    }

    #[test]
    fn pressing_restore_asks_rapidkoi_to_restore() {
        let pressed = Event::Message { msg: Msg::Restore, value: None };
        assert_eq!(NewTab::default().step(pressed).1, vec![Ask::restore_session("restore")]);
    }

    #[test]
    fn it_has_its_icon() {
        // Every app has one (PRP-060).
        let script = include_str!("../../../../scripts/newtab.sh");
        assert!(script.contains("--icon guest/rust/newtab/icon.svg"));
        assert!(std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/icon.svg")).is_file());
    }
}
