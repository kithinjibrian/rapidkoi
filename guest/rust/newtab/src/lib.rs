//! Rapidkoi's new tab (PRP-027): the app every empty tab runs, built into
//! `./main`. For now a still copy of its target design
//! (`docs/designs/newtab-target.md`, board A5), at the owner's request:
//! every name, place and notification on it is a placeholder, it asks
//! Rapidkoi for nothing, and nothing on it does anything yet.
//!
//! Contract 9's styles have no gradients and can't lay one thing over
//! another, so what needs either (the wordmark, faces with an app on their
//! corner, the beach, the washes) is a picture in `assets/`, packed by
//! `scripts/newtab.sh`, which writes `assets/apps/newtab.rkb`.

use rapidkoi::raw::boxed;
use rapidkoi::style::{
    Align, Background, Border, Color, Corner, Font, Layout, Sizing, Spacing, State as Look, Text,
    TextSize, Weight, When,
};
use rapidkoi::{column, image, row, spacer, text, App, Event, Fit, Next, Space, State, View};

/// Every picture the screen shows, each packed from `assets/` under its name.
const ASSETS: [&str; 15] = [
    "wordmark.svg",
    "restore-apps.svg",
    "face-amina.svg",
    "face-wanjiku.svg",
    "update-notes.svg",
    "notice-notes.svg",
    "beach.svg",
    "wash-notes.svg",
    "wash-counter.svg",
    "wash-settings.svg",
    "wash-fill.svg",
    "notes.svg",
    "counter.svg",
    "trip.svg",
    "settings.svg",
];

/// Each half of the page at its narrowest (the design's, at a 1280-pixel
/// window); both halves grow alike, so Notifications' columns and Recent's two
/// sides line up at any width.
const HALF: u16 = 496;
/// A small Recent card at its narrowest: two to a half.
const MINI: u16 = 240;

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
        let page = column([top(), notifications(), recent(), pinned()]).style([
            Spacing::padding_xy(Space::Px(32), Space::Px(36)),
            Spacing::gap(Space::Px(32)),
        ]);
        // Rapidkoi never scrolls an app's page (PRP-046): the page scrolls itself.
        column([page]).style([Layout::scroll(), Layout::grow()])
    }
}

rapidkoi::export!(NewTab);

/// A picture from the bundle, at its size.
fn picture(name: &str, w: u16, h: u16) -> V {
    debug_assert!(ASSETS.contains(&name), "{name} isn't packed");
    image(&format!("asset:{name}")).style([
        Sizing::width(w),
        Sizing::height(h),
        Layout::align(Align::Center),
    ])
}

/// The page's colour a step towards the text: cards, pills, hovers.
fn lift(by: f64) -> Color {
    Color::system("background").mix(Color::system("text"), by)
}

/// A section's name: small, bold and dim.
fn heading(s: &str) -> V {
    text(s).style([
        Font::size(TextSize::Small),
        Font::weight(Weight::Bold),
        Text::color(Color::system("text_dim")),
    ])
}

/// Small, dim text.
fn note(s: &str) -> V {
    text(s).style([
        Font::size(TextSize::Small),
        Text::color(Color::system("text_dim")),
    ])
}

/// A round button's look: the accent for the one to press first, else the surface.
fn pill(word: &str, first: bool) -> V {
    let (fill, ink) = if first {
        ("accent", "on_accent")
    } else {
        ("surface", "text")
    };
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
fn card(round: u16, hover_edge: bool) -> Vec<rapidkoi::style::Style> {
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

/// The wordmark, and Restore with the apps it brings back.
fn top() -> V {
    let restore = row([
        // The pill's text sits further from its edge than Restore does.
        boxed().style([Sizing::width(8)]),
        text("Restore previous session").style([Layout::align(Align::Center)]),
        picture("restore-apps.svg", 62, 26),
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
    row([
        picture("wordmark.svg", 176, 44).label("Rapidkoi"),
        spacer(),
        restore,
    ])
}

/// One notification: who or what it's from, its words (each bold or not), and its button if it has one.
fn notice(from: &str, words: &[(&str, bool)], detail: &str, button: Option<(&str, bool)>) -> V {
    let line = row(words.iter().map(|(w, bold)| {
        let t = text(w);
        if *bold {
            t.style([Font::weight(Weight::Bold)])
        } else {
            t
        }
    }));
    let mut parts = vec![
        picture(from, 44, 44),
        column([line, note(detail)]).style([
            Spacing::gap(Space::Px(3)),
            Layout::grow(),
            Layout::align(Align::Center),
        ]),
    ];
    if let Some((word, first)) = button {
        parts.push(pill(word, first));
    }
    row(parts).style(
        [
            vec![
                Sizing::min_width(HALF),
                Layout::grow(),
                Spacing::gap(Space::Px(14)),
                Spacing::padding_xy(Space::Px(12), Space::Px(14)),
            ],
            card(14, false),
        ]
        .concat(),
    )
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
    let head = row([
        heading("NOTIFICATIONS").style([Layout::align(Align::Center)]),
        count,
        spacer(),
        note("See all"),
    ])
    .style([Spacing::gap(Space::Sm)]);
    let invite = notice(
        "face-amina.svg",
        &[("Amina invited you to ", false), ("Sprint 3", true)],
        "Board · live now",
        Some(("Join", true)),
    );
    let shared = notice(
        "face-wanjiku.svg",
        &[("Wanjiku shared ", false), ("Mombasa, 12–15 Nov", true)],
        "Trip · 20 min ago",
        Some(("Open", false)),
    );
    let update = notice(
        "update-notes.svg",
        &[("Notes 2.5", true), (" is ready", false)],
        "notes.dafifi.net · 1 h ago",
        Some(("Update", false)),
    );
    let own = notice(
        "notice-notes.svg",
        &[("“Shopping list” is due today", false)],
        "Notes · 2 h ago",
        None,
    );
    column([
        head,
        row([invite, shared]).style([Spacing::gap(Space::Px(16))]),
        row([update, own]).style([Spacing::gap(Space::Px(16))]),
    ])
    .style([Spacing::gap(Space::Px(12))])
}

/// A small Recent card: its picture area, title and details.
fn mini(wash: &str, title: &str, detail: &str) -> V {
    // The wash fades out before its right edge, onto the colour the area grows in.
    // The filler is cut to the rest of the area: wide, it makes the card ask for
    // more room than it gets, so cards beside the beach share a row evenly.
    let fill = image("asset:wash-fill.svg").fit(Fit::Cover).style([
        Sizing::height(84),
        Layout::grow(),
        Border::corner(Corner::Px(8)),
    ]);
    let area = row([
        picture(wash, 222, 84).style([Layout::align(Align::Start)]),
        fill,
    ])
    .style([
        Background::color(Color::rgb(0x1b, 0x1b, 0x1b)),
        Border::corner(Corner::Px(8)),
    ]);
    column([
        area,
        column([
            text(title).style([Font::weight(Weight::Bold), Text::truncate()]),
            note(detail),
        ])
        .style([
            Spacing::padding_xy(Space::None, Space::Px(4)),
            Spacing::gap(Space::Px(3)),
        ]),
    ])
    .style(
        [
            vec![
                Sizing::min_width(MINI),
                Layout::grow(),
                Spacing::padding(Space::Px(8)),
                Spacing::gap(Space::Px(10)),
            ],
            card(14, true),
        ]
        .concat(),
    )
}

/// Recent: the newest place large in the left half, the next four small in the right.
fn recent() -> V {
    let large = column([
        // As wide as the card, cut to its height (`beach.svg` is drawn wide for this).
        image("asset:beach.svg")
            .fit(Fit::Cover)
            .style([Sizing::height(240), Border::corner(Corner::Px(10))]),
        column([
            text("Mombasa, 12–15 Nov")
                .style([Font::size(TextSize::Rem(1.45)), Font::weight(Weight::Bold)]),
            text("3 nights · 2 bookings confirmed"),
            row([
                picture("trip.svg", 18, 18),
                note("Trip · localhost:8000/trips/12 · 2 min ago"),
            ])
            .style([Spacing::gap(Space::Sm)]),
        ])
        .style([
            Spacing::padding_xy(Space::None, Space::Px(8)),
            Spacing::gap(Space::Px(6)),
        ]),
    ])
    .style(
        [
            vec![
                Sizing::min_width(HALF),
                Layout::grow(),
                Spacing::padding(Space::Px(12)),
                Spacing::gap(Space::Px(14)),
            ],
            card(16, true),
        ]
        .concat(),
    );
    let small = column([
        row([
            mini(
                "wash-notes.svg",
                "Shopping list",
                "4 items left · Notes · 1 h ago",
            ),
            mini("wash-counter.svg", "Counter", "Counter · Yesterday"),
        ])
        // The two rows share the large card's height, as the design's grid does.
        .style([Spacing::gap(Space::Px(16)), Layout::grow()]),
        row([
            mini(
                "wash-notes.svg",
                "Ideas for the new tab",
                "Notes · Yesterday",
            ),
            mini("wash-settings.svg", "Appearance", "Settings · Monday"),
        ])
        .style([Spacing::gap(Space::Px(16)), Layout::grow()]),
    ])
    .style([
        Sizing::min_width(HALF),
        Layout::grow(),
        Spacing::gap(Space::Px(16)),
    ]);
    column([
        heading("RECENT"),
        row([large, small]).style([Spacing::gap(Space::Px(16))]),
    ])
    .style([Spacing::gap(Space::Px(12))])
}

/// Pinned: one line of each app's icon and name, and See all.
fn pinned() -> V {
    let tile = |icon: &str, name: &str| {
        column([
            picture(icon, 56, 56),
            text(name).style([Font::size(TextSize::Small), Layout::align(Align::Center)]),
        ])
        .style([Sizing::width(80), Spacing::gap(Space::Sm)])
    };
    // One line of pins, then See all (the owner's choice): contract 9's rows
    // don't wrap, and the app doesn't know the window's width.
    column([
        row([heading("PINNED"), spacer(), note("See all")]),
        row([
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
            assert!(
                std::path::Path::new(&format!("{dir}{name}")).is_file(),
                "assets/{name} is missing"
            );
            assert!(
                script.contains(&format!("--asset {name}=")),
                "scripts/newtab.sh doesn't pack {name}"
            );
        }
    }

    #[test]
    fn nothing_on_it_asks_or_changes() {
        // A still copy: whatever happens, the state stays and nothing is asked.
        let _ = NewTab::default().handle(Event::Opened);
        let _ = NewTab::default().handle(Event::Resume);
    }
}
