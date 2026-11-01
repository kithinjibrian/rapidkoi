//! Rapidkoi's new tab (PRP-027): the app every empty tab runs, built into
//! `./main`. A copy of its target design (`docs/designs/newtab-target.md`,
//! board A5), becoming real a part at a time. Real: Restore, which asks for
//! your last session each time the tab is shown and shows its apps' icons
//! (PRP-060: `icon:PLACE`, drawn by Rapidkoi); Pinned, your pins, each
//! opening its app (PRP-061); and Recent, your recent places, each opening a
//! fresh window there (PRP-062). Still placeholders, doing nothing:
//! notifications.
//!
//! Drawn with contract 10's looks (PRP-059): gradients for the washes, the
//! page's glow and Rapidkoi's mark; layers for a face or an icon with
//! something on its corner, and for Restore's overlapping icons; wraps for
//! the two-column sections, which fall to one column on a narrow window;
//! a fitting row for Pinned. What stays a picture in `assets/` (packed by
//! `scripts/newtab.sh`): the wordmark (GTK can't fill text with a
//! gradient), and the notifications' placeholder icons.

use minicbor::{Decode, Encode};
use rapidkoi::raw::boxed;
use rapidkoi::style::{
    Align, Background, Border, Color, Corner, Font, Layout, Place, Sizing, Spacing, State as Look, Style, Text,
    TextSize, Weight, When,
};
use rapidkoi::{
    column, fit, image, layers, row, spacer, text, text_of, wrap, App, Ask, Event, Message, Next, Recent, Session,
    Space, State, View,
};

/// Every picture the screen shows, each packed from `assets/` under its name.
const ASSETS: [&str; 4] = ["wordmark.svg", "notes.svg", "trip.svg", "board.svg"];

/// The card colour behind a badge's ring, so the ring reads as a gap.
const CARD: (u8, u8, u8) = (0x1e, 0x1e, 0x1e);
/// The Restore pill's colour behind its icons' rings.
const PILL: (u8, u8, u8) = (0x23, 0x23, 0x23);

/// What the new tab's screen sends.
#[derive(Debug, Clone, PartialEq, Message)]
enum Msg {
    /// Bring back the last session.
    Restore,
    /// Open the pin at this place in your pins.
    OpenPin(u32),
    /// Open your recent place at this index (PRP-062).
    OpenRecent(u32),
}

/// One of your pins as the new tab shows it: its name and its icon's source (`icon:PLACE`).
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Pin {
    #[n(0)]
    name: String,
    #[n(1)]
    icon: String,
}

type V = View<Msg>;

#[derive(Default, State)]
struct NewTab {
    /// The last session's apps' icons, in order (`icon:PLACE`), while there's one to restore.
    restore: Option<Vec<String>>,
    /// Your pins, in your order, as last read.
    pins: Vec<Pin>,
    /// Your recent places, newest first, as last read (PRP-062).
    recent: Vec<Spot>,
}

/// One of your recent places as the new tab shows it.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Spot {
    #[n(0)]
    name: String,
    #[n(1)]
    icon: String,
    #[n(2)]
    address: String,
    #[n(3)]
    title: Option<String>,
    #[n(4)]
    seen: i64,
}

impl App for NewTab {
    type Msg = Msg;

    fn handle(self, event: Event<Msg>) -> Next<Self> {
        // The new tab has no address of its own to put fields in.
        self.step(event).into()
    }

    fn view(&self) -> V {
        // The page fills the window's width (the owner's choice over the design's cap).
        let mut sections = vec![top(&self.restore), notifications()];
        sections.extend(recent(&self.recent));
        sections.extend(pinned(&self.pins));
        let page =
            column(sections).style([Spacing::padding_xy(Space::Px(32), Space::Px(36)), Spacing::gap(Space::Px(32))]);
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
            // Each time it's shown, or told something changed: the session and your pins may have.
            Event::Opened | Event::Resume | Event::Change { .. } => {
                (self, vec![Ask::session("session"), Ask::pins("pins"), Ask::recent("recent")])
            }
            Event::Data { msg, cbor } if msg == "pins" => {
                let pins = rapidkoi::Place::list(&cbor).unwrap_or_default();
                (
                    NewTab { pins: pins.into_iter().map(|p| Pin { name: p.name, icon: p.icon }).collect(), ..self },
                    vec![],
                )
            }
            Event::Data { msg, cbor } if msg == "session" => {
                let icons = Session::read(&cbor).map(|s| s.apps.into_iter().map(|p| p.icon).collect());
                (NewTab { restore: icons, ..self }, vec![])
            }
            // No session to restore.
            Event::Answer { msg, .. } if msg == "session" => (NewTab { restore: None, ..self }, vec![]),
            Event::Message { msg: Msg::Restore, .. } => (self, vec![Ask::restore_session("restore")]),
            Event::Message { msg: Msg::OpenPin(i), .. } => (self, vec![Ask::open_pin("open", i)]),
            Event::Data { msg, cbor } if msg == "recent" => {
                let places = Recent::read(&cbor).unwrap_or_default();
                let spots = places
                    .into_iter()
                    .map(|p| Spot { name: p.name, icon: p.icon, address: p.address, title: p.title, seen: p.seen })
                    .collect();
                (NewTab { recent: spots, ..self }, vec![])
            }
            Event::Message { msg: Msg::OpenRecent(i), .. } => (self, vec![Ask::open_recent("open", i)]),
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

/// A card's picture area until apps give one (a later PRP): the app's icon
/// on a wash from the top-left, in the theme's colours, fading into the card.
fn wash(icon: &str, height: u16, size: u16, centred: bool) -> V {
    let place = if centred { Align::Center } else { Align::Start };
    let middle = if centred { vec![spacer()] } else { vec![] };
    let mut kids = middle.clone();
    kids.push(image(icon).style([Sizing::width(size), Sizing::height(size), Layout::align(place)]));
    kids.extend(middle);
    column(kids).style([
        Sizing::height(height),
        Spacing::padding(Space::Px(12)),
        Border::corner(Corner::Px(10)),
        Background::color(lift(0.06)),
        Background::ellipse(0.0, 0.0, 1.1, 1.5, [(lift(0.16), 0.0), (lift(0.06), 0.9)]),
    ])
}

/// When a place was last open: Rapidkoi fills it in and keeps it current ("2 min ago").
fn when(seen: i64) -> rapidkoi::Part {
    rapidkoi::time(seen).relative()
}

/// Small, dim words with a time at their end.
fn note_at(words: &str, seen: i64) -> V {
    text_of([words.into(), when(seen)]).style([Font::size(TextSize::Small), Text::color(Color::system("text_dim"))])
}

/// A small Recent card: its picture area, title, app and when.
fn mini(i: usize, p: &Spot) -> V {
    column([
        wash(&p.icon, 84, 44, false),
        column([
            text(p.title.as_deref().unwrap_or(&p.name)).style([Font::weight(Weight::Bold), Text::truncate()]),
            note_at(&format!("{} · ", p.name), p.seen),
        ])
        .style([Spacing::padding_xy(Space::None, Space::Px(4)), Spacing::gap(Space::Px(3))]),
    ])
    .style([vec![Spacing::padding(Space::Px(8)), Spacing::gap(Space::Px(10))], card(14, true)].concat())
    .on_press(Msg::OpenRecent(i as u32))
}

/// Recent: your newest place large in the left half, the next four small in
/// the right, each opening a fresh window there; nothing when you have none.
fn recent(places: &[Spot]) -> Option<V> {
    let newest = places.first()?;
    let large = column([
        wash(&newest.icon, 240, 88, true),
        column([
            text(newest.title.as_deref().unwrap_or(&newest.name)).style([
                Font::size(TextSize::Rem(1.45)),
                Font::weight(Weight::Bold),
                Text::truncate(),
            ]),
            row([
                image(&newest.icon).style([Sizing::width(18), Sizing::height(18)]),
                note_at(&format!("{} · {} · ", newest.name, newest.address), newest.seen),
            ])
            .style([Spacing::gap(Space::Sm)]),
        ])
        .style([Spacing::padding_xy(Space::None, Space::Px(8)), Spacing::gap(Space::Px(6))]),
    ])
    .style([vec![Spacing::padding(Space::Px(12)), Spacing::gap(Space::Px(14))], card(16, true)].concat())
    .on_press(Msg::OpenRecent(0));
    let small = halves(places.iter().enumerate().skip(1).take(4).map(|(i, p)| mini(i, p)));
    Some(column([heading("RECENT"), halves([large, small])]).style([Spacing::gap(Space::Px(12))]))
}

/// Pinned: your pins, as many icons and names as fit on a line, each opening
/// its app, and See all; nothing when you have none.
fn pinned(pins: &[Pin]) -> Option<V> {
    if pins.is_empty() {
        return None;
    }
    let tiles = pins.iter().enumerate().map(|(i, p)| {
        column([
            image(&p.icon).style([Sizing::width(56), Sizing::height(56), Layout::align(Align::Center)]),
            text(&p.name).style([Font::size(TextSize::Small), Layout::align(Align::Center), Text::truncate()]),
        ])
        .style([
            Sizing::width(80),
            Spacing::gap(Space::Sm),
            Border::corner(Corner::Px(12)),
            When(Look::Hover, [Background::color(lift(0.07))]),
        ])
        .on_press(Msg::OpenPin(i as u32))
    });
    // One line of pins, then See all (the owner's choice): the line ends
    // cleanly at the window's edge, the rest hidden.
    Some(
        column([row([heading("PINNED"), spacer(), note("See all")]), fit(tiles).style([Spacing::gap(Space::Lg)])])
            .style([Spacing::gap(Space::Px(12))]),
    )
}

#[cfg(test)]
mod tests;
