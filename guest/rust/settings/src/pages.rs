//! Settings' screen (PRP-075), as the canvas drew it: categories down the
//! side, Find and the page beside them, a details column beside You's and
//! Publishers' lists. Rapidkoi's names only (`surface`, `text_dim`,
//! `accent`, …), so it follows the theme, contrast and text size; the theme
//! cards' small drawings are the only raw colours, each the other theme's.

use crate::find::{Page, PAGES};
use crate::lists::{publishers, you};
use crate::{Msg, Settings, STOPS};
use rapidkoi::raw::{boxed, Role};
use rapidkoi::style::{Align, Background, Border, Color, Corner, Font, Layout, Sizing, Spacing, TextSize, Weight};
use rapidkoi::{column, image, input, row, slider, spacer, text, wrap, Space, View};

type V = View<Msg>;

/// Every picture the screen shows, each packed from `assets/` under its name.
pub const ASSETS: [&str; 6] = ["brush.svg", "startup.svg", "user.svg", "shield.svg", "info.svg", "search.svg"];

/// The page's width, Find's included; the side's.
const CONTENT: u16 = 680;
const SIDE: u16 = 240;

pub(crate) fn sys(name: &str) -> Color {
    Color::system(name)
}

pub(crate) fn icon(name: &str, size: u16, colour: &str) -> V {
    debug_assert!(crate::pages::ASSETS.contains(&name), "{name} isn't packed");
    mid(image(&format!("asset:{name}")).color(sys(colour)).style([Sizing::width(size), Sizing::height(size)]))
}

/// Centred across its parent: down a row, along a column. A box's `align`
/// is where it sits in its parent, never where its children sit.
pub(crate) fn mid(v: V) -> V {
    v.style([Layout::align(Align::Center)])
}

pub(crate) fn dim(s: &str) -> V {
    text(s).dim()
}

/// The whole screen.
pub fn screen(s: &Settings) -> V {
    let page = s.shown();
    let mut main = vec![find_box(s), title(page)];
    match page {
        None => main.push(dim("No results")),
        Some(p) => {
            let (body, details) = body(s, p);
            let list = column(body).spacing(Space::Lg);
            match details {
                // Beside the details, the list is exactly Find's width and the
                // details take the rest: no gap between them.
                Some(d) => main.push(row([list.style([Sizing::width(CONTENT)]), d]).spacing(Space::Lg)),
                None => main.push(list.style([Sizing::max_width(CONTENT), Layout::grow()])),
            }
        }
    }
    row([side(s, page), column(main).spacing(Space::Lg).style([Layout::grow()])])
        .spacing(Space::Xl)
        .padding(Space::Xl)
        // Rapidkoi never scrolls an app's page (PRP-046): a short window scrolls this.
        .style([Layout::scroll(), Layout::grow()])
        .screen_title("Settings")
}

fn side(s: &Settings, page: Option<Page>) -> V {
    let mut items = vec![text("Settings").title().style([Spacing::padding_xy(Space::None, Space::Md)])];
    for p in PAGES {
        if p.starts_group() {
            items.push(line());
        }
        let on = page == Some(p);
        let faded = !p.matches(&s.find);
        let mut item = boxed()
            .role(Role::Tab)
            .selected(on)
            .on_press(Msg::Go(p.index()))
            .style([
                Layout::row(),
                Spacing::gap(Space::Md),
                Spacing::padding_xy(Space::Sm, Space::Md),
                Border::corner(Corner::Round),
            ])
            .children([
                icon(p.icon(), 20, if on { "accent" } else { "text" }),
                text(p.name()).style([Font::weight(if on { Weight::Medium } else { Weight::Normal })]),
            ]);
        if on {
            item = item.style([Background::color(sys("surface"))]);
        }
        if faded {
            item = item.style([rapidkoi::style::Effects::opacity(0.4)]);
        }
        items.push(item);
    }
    column(items).role(Role::TabList).label("Settings").spacing(Space::Xs).style([Sizing::width(SIDE)])
}

fn line() -> V {
    boxed().style([Sizing::height(1), Background::color(sys("border")), Spacing::padding_xy(Space::None, Space::Md)])
}

fn find_box(s: &Settings) -> V {
    let field = input(&s.find).placeholder("Find in Settings").label("Find in Settings").on_change(Msg::Find).style([
        Layout::grow(),
        Background::color(Color::rgba(0, 0, 0, 0)),
        Border::all(0, Color::rgba(0, 0, 0, 0)),
    ]);
    row([icon("search.svg", 18, "text_dim"), field]).spacing(Space::Sm).style([
        Spacing::padding_xy(Space::Sm, Space::Lg),
        Background::color(sys("surface")),
        Border::corner(Corner::Round),
        Sizing::max_width(CONTENT),
    ])
}

fn title(page: Option<Page>) -> V {
    match page {
        Some(p) => row([icon(p.icon(), 24, "accent"), mid(text(p.name()).title())]).spacing(Space::Md),
        None => text("Settings").title(),
    }
}

/// A page's sections, and its details column when it has one.
fn body(s: &Settings, p: Page) -> (Vec<V>, Option<V>) {
    match p {
        Page::Appearance => {
            (vec![section("Theme", themes(s)), section("Reading", group(vec![contrast(s), size(s)]))], None)
        }
        Page::Startup => (
            vec![section("When Rapidkoi opens", group(starts(s)).role(Role::TabList).label("When Rapidkoi opens"))],
            None,
        ),
        Page::You => you(s),
        Page::Publishers => publishers(s),
        Page::About => (vec![about()], None),
    }
}

/// A dim heading over its rows.
pub(crate) fn section(heading: &str, rows: V) -> V {
    column([dim(heading).style([Spacing::padding_xy(Space::None, Space::Md)]), rows]).spacing(Space::Xs)
}

/// Rows in one rounded group, a line between each.
pub(crate) fn group(rows: Vec<V>) -> V {
    let mut out = Vec::new();
    for (i, r) in rows.into_iter().enumerate() {
        if i > 0 {
            out.push(boxed().style([Sizing::height(1), Background::color(sys("border"))]));
        }
        out.push(r);
    }
    column(out).style([Background::color(sys("surface")), Border::corner(Corner::Large)])
}

pub(crate) fn padded(v: V) -> V {
    v.style([Spacing::padding(Space::Md)])
}

fn themes(s: &Settings) -> V {
    let card = |name: &str, light: bool| {
        // The other theme's own colours, so the card shows it whatever's on now.
        let (frame, page, ink, control, accent) = if light {
            (
                Color::rgb(0xe4, 0xe4, 0xe7),
                Color::rgb(0xff, 0xff, 0xff),
                Color::rgb(0x1a, 0x1a, 0x1a),
                Color::rgb(0xec, 0xec, 0xec),
                Color::rgb(0xc2, 0x54, 0x00),
            )
        } else {
            (
                Color::rgb(0x24, 0x24, 0x24),
                Color::rgb(0x16, 0x16, 0x16),
                Color::rgb(0xe8, 0xe8, 0xe8),
                Color::rgb(0x2e, 0x2e, 0x2e),
                Color::rgb(0xff, 0x77, 0x00),
            )
        };
        let on = s.light == light;
        let content = column([
            boxed().style([
                Sizing::width(64),
                Sizing::height(6),
                Background::color(ink),
                Border::corner(Corner::Small),
            ]),
            boxed().style([Sizing::height(22), Background::color(control), Border::corner(Corner::Small)]),
            boxed().style([
                Sizing::width(44),
                Sizing::height(10),
                Background::color(accent),
                Border::corner(Corner::Small),
            ]),
        ])
        .spacing(Space::Xs)
        .padding(Space::Sm)
        .style([Background::color(page), Layout::grow()]);
        // The header above the page, the sidebar beside it: the frame's L.
        let drawing = column([
            boxed().style([Sizing::height(14)]),
            row([content, boxed().style([Sizing::width(12)])]).style([Layout::grow()]),
        ])
        .style([Sizing::height(96), Background::color(frame), Border::corner(Corner::Medium)]);
        boxed()
            .role(Role::Tab)
            .selected(on)
            .label(name)
            .on_press(Msg::Theme(light))
            .style([
                Spacing::gap(Space::Md),
                Spacing::padding(Space::Md),
                Background::color(sys("surface")),
                Border::corner(Corner::Large),
                Border::all(2, if on { sys("accent") } else { Color::rgba(0, 0, 0, 0) }),
            ])
            .children([drawing, row([dot(on), mid(text(name))]).spacing(Space::Sm)])
    };
    wrap([card("Dark", false), card("Light", true)])
        .spacing(Space::Md)
        .style([Layout::equal(), Layout::per_line(2)])
        .role(Role::TabList)
        .label("Theme")
}

/// A choice's round mark: filled when it's the one.
fn dot(on: bool) -> V {
    let mark = boxed().style([
        Sizing::width(16),
        Sizing::height(16),
        Border::corner(Corner::Round),
        Layout::align(Align::Center),
    ]);
    if on {
        mark.style([Border::all(5, sys("accent")), Background::color(sys("on_accent"))])
    } else {
        mark.style([Border::all(2, sys("text_dim"))])
    }
}

fn contrast(s: &Settings) -> V {
    let on = s.more_contrast;
    let knob = boxed().style([
        Sizing::width(20),
        Sizing::height(20),
        Border::corner(Corner::Round),
        Background::color(if on { sys("on_accent") } else { sys("text") }),
    ]);
    let track = row(if on { vec![spacer(), knob] } else { vec![knob, spacer()] }).style([
        Sizing::width(46),
        Sizing::height(26),
        Spacing::padding(Space::Px(3)),
        Border::corner(Corner::Round),
        Background::color(if on { sys("accent") } else { sys("surface").mix(sys("text"), 0.25) }),
    ]);
    padded(
        boxed()
            .role(Role::Switch)
            .checked(on)
            .label("More contrast")
            .on_press(Msg::Contrast)
            .style([Layout::row()])
            .children([mid(text("More contrast")), spacer(), mid(track)]),
    )
}

fn size(s: &Settings) -> V {
    let at = s.stop();
    // The stops' numbers spread end to end, as the slider's stops are: room
    // between each, none before the first or after the last.
    let mut marks = Vec::new();
    for (i, n) in STOPS.iter().enumerate() {
        if i > 0 {
            marks.push(spacer());
        }
        let label = text(&n.to_string()).style([Font::size(TextSize::Small)]);
        marks.push(if i == at { label.style([Font::weight(Weight::Bold)]) } else { label.dim() });
    }
    let track =
        column([slider(0, (STOPS.len() - 1) as i64, at as i64).label("Text size").on_change(Msg::Size), row(marks)])
            .spacing(Space::Xs)
            .style([Layout::grow()]);
    padded(
        column([
            row([text("Text size"), spacer(), dim(&format!("{}%", s.text_size))]),
            row([
                mid(dim("A").style([Font::size(TextSize::Small)])),
                track,
                mid(dim("A").style([Font::size(TextSize::Heading)])),
            ])
            .spacing(Space::Md),
        ])
        .spacing(Space::Sm),
    )
}

fn starts(s: &Settings) -> Vec<V> {
    [("Open a new tab", false), ("Restore your last session", true)]
        .into_iter()
        .map(|(words, restore)| {
            let on = s.restore == restore;
            padded(
                boxed()
                    .role(Role::Tab)
                    .selected(on)
                    .label(words)
                    .on_press(Msg::Start(restore))
                    .style([Layout::row(), Spacing::gap(Space::Md)])
                    .children([dot(on), mid(text(words))]),
            )
        })
        .collect()
}

fn about() -> V {
    let version = option_env!("RAPIDKOI_VERSION").unwrap_or("");
    group(vec![
        padded(
            row([
                boxed().style([
                    Sizing::width(48),
                    Sizing::height(48),
                    Border::corner(Corner::Large),
                    Background::color(sys("accent")),
                ]),
                mid(column([text("Rapidkoi").style([Font::weight(Weight::Bold)]), dim(version)])),
            ])
            .spacing(Space::Md),
        ),
        padded(row([text("Apps' contract"), spacer(), dim(&rapidkoi::__CONTRACT_VERSION.to_string())])),
    ])
}
