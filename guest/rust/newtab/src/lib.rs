//! Rapidkoi's new tab (PRP-027): the app every empty tab runs, built into
//! `./main`. A copy of its target design (`docs/designs/newtab-target.md`,
//! board A5), becoming real a part at a time. Real: Restore, which asks for
//! your last session each time the tab is shown and shows its apps' icons
//! (PRP-060: `icon:PLACE`, drawn by Rapidkoi); Pinned, your pins, each
//! opening its app (PRP-061); and Recent, your recent places, each opening a
//! fresh window there (PRP-062); and Notifications, what apps told you,
//! while something is unread (PRP-065, `notices.rs`).
//!
//! Drawn with contract 10's looks (PRP-059): gradients for the washes, the
//! page's glow and Rapidkoi's mark; layers for a face or an icon with
//! something on its corner, and for Restore's overlapping icons; wraps for
//! the two-column sections, which fall to one column on a narrow window;
//! a fitting row for Pinned. What stays a picture in `assets/` (packed by
//! `scripts/newtab.sh`): the wordmark (GTK can't fill text with a
//! gradient).

use minicbor::{Decode, Encode};
use rapidkoi::raw::boxed;
use rapidkoi::style::{
    Align, Background, Border, Color, Corner, Font, Layout, Place, Sizing, Spacing, State as Look, Style, Text,
    TextSize, Weight, When,
};
use rapidkoi::{
    column, fit, image, layers, row, spacer, text, text_of, wrap, App, Ask, Event, Message, Next, Notifications,
    Recent, Session, Space, State, View,
};

mod notices;
use notices::Alert;

/// Every picture the screen shows, each packed from `assets/` under its name.
const ASSETS: [&str; 2] = ["wordmark.svg", "x.svg"];

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
    /// Open your notification at this index, as they were listed (PRP-065).
    OpenNotice(u32),
    /// Remove your notification at this index.
    DismissNotice(u32),
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
    /// Your notifications, unread first, as last read (PRP-065).
    notices: Vec<Alert>,
    /// How many of them are unread.
    unread: u32,
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
    /// Its app's primary colour, `#rrggbb` (PRP-063).
    #[n(5)]
    colour: String,
}

impl App for NewTab {
    type Msg = Msg;

    fn handle(self, event: Event<Msg>) -> Next<Self> {
        // The new tab has no address of its own to put fields in.
        self.step(event).into()
    }

    fn view(&self) -> V {
        // The page fills the window's width (the owner's choice over the design's cap).
        let mut sections = vec![top(&self.restore)];
        sections.extend(notices::section(self.unread, &self.notices));
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
                let asks = vec![Ask::session("session"), Ask::pins("pins"), Ask::recent("recent")];
                (self, [asks, vec![Ask::notifications("notifications")]].concat())
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
                    .map(|p| Spot {
                        name: p.name,
                        icon: p.icon,
                        address: p.address,
                        title: p.title,
                        seen: p.seen,
                        colour: p.primary_color,
                    })
                    .collect();
                (NewTab { recent: spots, ..self }, vec![])
            }
            Event::Message { msg: Msg::OpenRecent(i), .. } => (self, vec![Ask::open_recent("open", i)]),
            Event::Data { msg, cbor } if msg == "notifications" => {
                let read = Notifications::read(&cbor).unwrap_or_default();
                let notices = read.items.into_iter().map(Alert::from).collect();
                (NewTab { notices, unread: read.unread, ..self }, vec![])
            }
            Event::Message { msg: Msg::OpenNotice(i), .. } => (self, vec![Ask::open_notification("open", i)]),
            Event::Message { msg: Msg::DismissNotice(i), .. } => (self, vec![Ask::dismiss_notification("dismiss", i)]),
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

/// Two equal columns that fall to one on a narrow window.
fn halves(children: impl IntoIterator<Item = V>) -> V {
    wrap(children).style([Layout::equal(), Layout::per_line(2), Spacing::gap(Space::Px(16))])
}

/// A card's picture area until apps give one (a later PRP): the app's icon
/// at the top-left, on its primary colour washing in from that corner and
/// fading before the far edges (PRP-063, as board A5's washes).
fn wash(icon: &str, colour: &str, height: u16, size: u16, room: u16) -> V {
    let (r, g, b) = rgb_of(colour);
    column([image(icon).style([Sizing::width(size), Sizing::height(size), Layout::align(Align::Start)])]).style([
        Sizing::height(height),
        Spacing::padding(Space::Px(room)),
        Border::corner(Corner::Px(10)),
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

/// A `#rrggbb` colour's channels; Rapidkoi checked it, so a bad one is grey.
fn rgb_of(c: &str) -> (u8, u8, u8) {
    let byte = |i: usize| c.get(i..i + 2).and_then(|h| u8::from_str_radix(h, 16).ok()).unwrap_or(0x71);
    (byte(1), byte(3), byte(5))
}

/// When a place was last open: Rapidkoi fills it in and keeps it current ("2 min ago").
fn when(seen: i64) -> rapidkoi::Part {
    rapidkoi::time(seen).relative()
}

/// Small, dim words: an app and, when it has one, an address, then when it
/// was open, left out when it isn't known (a time of 0).
fn note_at(words: &[&str], seen: i64) -> V {
    let mut parts: Vec<rapidkoi::Part> = Vec::new();
    for w in words.iter().filter(|w| !w.is_empty()) {
        if !parts.is_empty() {
            parts.push(" · ".into());
        }
        parts.push((*w).into());
    }
    if seen > 0 {
        parts.push(" · ".into());
        parts.push(when(seen));
    }
    text_of(parts).style([Font::size(TextSize::Small), Text::color(Color::system("text_dim"))])
}

/// A small Recent card: its picture area, title, app and when.
fn mini(i: usize, p: &Spot) -> V {
    column([
        wash(&p.icon, &p.colour, 84, 44, 12),
        column([
            text(p.title.as_deref().unwrap_or(&p.name)).style([Font::weight(Weight::Bold), Text::truncate()]),
            note_at(&[&p.name], p.seen),
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
        wash(&newest.icon, &newest.colour, 240, 88, 28),
        column([
            text(newest.title.as_deref().unwrap_or(&newest.name)).style([
                Font::size(TextSize::Rem(1.45)),
                Font::weight(Weight::Bold),
                Text::truncate(),
            ]),
            row([
                image(&newest.icon).style([Sizing::width(18), Sizing::height(18)]),
                note_at(&[&newest.name, &newest.address], newest.seen),
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
