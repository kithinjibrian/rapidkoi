//! Rapidkoi's new tab (PRP-027): the app every empty tab runs, built into
//! `./main`. For now a still copy of its target design
//! (`docs/designs/newtab-target.md`, board A5), at the owner's request:
//! every name, place and notification on it is a placeholder, it asks
//! Rapidkoi for nothing, and nothing on it does anything yet.
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
use rapidkoi::{column, fit, image, layers, row, spacer, text, wrap, App, Event, Fit, Next, Space, State, View};

/// Every picture the screen shows, each packed from `assets/` under its name.
const ASSETS: [&str; 7] =
    ["wordmark.svg", "beach.svg", "notes.svg", "counter.svg", "trip.svg", "settings.svg", "board.svg"];

/// The card colour behind a badge's ring, so the ring reads as a gap.
const CARD: (u8, u8, u8) = (0x1e, 0x1e, 0x1e);
/// The Restore pill's colour behind its icons' rings.
const PILL: (u8, u8, u8) = (0x23, 0x23, 0x23);

type V = View<String>;

#[derive(Default, State)]
struct NewTab {}

impl App for NewTab {
    // Nothing on the screen sends anything yet.
    type Msg = String;

    fn handle(self, _event: Event<String>) -> Next<Self> {
        Next::state(self)
    }

    fn view(&self) -> V {
        // The page fills the window's width (the owner's choice over the design's cap).
        let page = column([top(), notifications(), recent(), pinned()])
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

/// The wordmark, and Restore with the apps it brings back.
fn top() -> V {
    // Notes and Trip, then Counter, which has no icon: its initial. Each
    // overlaps the one before, by its place in a box narrower than the three.
    let apps = layers([
        boxed().style([Sizing::width(62), Sizing::height(26)]),
        ringed("notes.svg", 26, rgb(PILL)).style([Layout::place(Place::Start)]),
        ringed("trip.svg", 26, rgb(PILL)).style([Layout::place(Place::Center)]),
        disc(
            "C",
            26,
            vec![Background::color(Color::rgb(0x6d, 0x28, 0xd9))],
            Color::rgb(255, 255, 255),
            TextSize::Rem(0.75),
            Some(rgb(PILL)),
        )
        .style([Layout::place(Place::End)]),
    ])
    .style([Layout::align(Align::Center)]);
    let restore = row([
        // The pill's text sits further from its edge than Restore does.
        boxed().style([Sizing::width(8)]),
        text("Restore previous session").style([Layout::align(Align::Center)]),
        apps,
        note("+2").style([Layout::align(Align::Center)]),
        pill("Restore", true),
    ])
    .style([
        Spacing::gap(Space::Md),
        Spacing::padding(Space::Px(6)),
        Background::color(lift(0.06)),
        Border::all(1, Color::system("border")),
        Border::corner(Corner::Round),
        Layout::align(Align::Center),
    ]);
    row([picture("wordmark.svg", 176, 44).label("Rapidkoi"), spacer(), restore])
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

    #[test]
    fn nothing_on_it_asks_or_changes() {
        // A still copy: whatever happens, the state stays and nothing is asked.
        let _ = NewTab::default().handle(Event::Opened);
        let _ = NewTab::default().handle(Event::Resume);
    }
}
