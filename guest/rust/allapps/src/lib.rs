//! Rapidkoi's All apps (PRP-067, DECISION-094): a built-in app, packed into
//! `./main` like Settings. It lists every app the person has opened, A–Z or
//! last used first, with a search box: typing filters by name and Enter
//! opens the first match. When two apps share a name, each also shows where
//! it's from. Each app's details (on a right-click, PRP-068) show what can be done with it:
//! open, pin or unpin, its permissions, Forget. Forget and permissions only
//! open Rapidkoi's own panel on the app, where the person decides.
//!
//! It reads with the built-in `read_apps`, opens with `open_apps`, and pins
//! with `manage_pins` (`scripts/allapps.sh` declares them). It asks for an
//! app by its index in Rapidkoi's A–Z list, whatever order it shows.
//!
//! Build and pack: `scripts/allapps.sh`, which writes `assets/apps/allapps.rkb`.

use minicbor::{Decode, Encode};
use rapidkoi::raw::{boxed, Role};
use rapidkoi::style::{
    Align, Background, Border, Color, Corner, Font, Layout, Place, Sizing, Spacing, State as Look, Text,
    TextSize, Weight, When,
};
use rapidkoi::{
    column, image, input, layers, row, spacer, text, text_of, wrap, App, Apps, Ask, Event, Message, Next, Space,
    State, View,
};

/// What All apps' screen sends.
#[derive(Debug, Clone, PartialEq, Message)]
enum Msg {
    /// The search box changed: its text is the event's value.
    Typed,
    /// Enter in the search box: open the first match.
    Submit,
    /// Last used first (`true`), or A–Z.
    Order(bool),
    /// Open the app at this index in Rapidkoi's list.
    Open(u32),
    /// Show this app's details; again, close them.
    Details(u32),
    Pin(u32),
    Unpin(u32),
    Permissions(u32),
    Forget(u32),
}

type V = View<Msg>;

/// Every picture the screen shows, each packed from `assets/` under its name.
const ASSETS: [&str; 2] = ["search.svg", "pin.svg"];

/// One of the person's apps, as All apps keeps it.
#[derive(Debug, Clone, PartialEq, Eq, Encode, Decode)]
struct Item {
    /// Its index in Rapidkoi's A–Z list: what every ask names it by.
    #[n(0)]
    at: u32,
    #[n(1)]
    name: String,
    /// `icon:PLACE`: Rapidkoi draws it.
    #[n(2)]
    icon: String,
    /// A host, or a file's name; empty for Rapidkoi's own.
    #[n(3)]
    from: String,
    #[n(4)]
    address: String,
    /// The host that signed it; empty when unsigned.
    #[n(5)]
    publisher: String,
    #[n(6)]
    signed: bool,
    #[n(7)]
    pinned: bool,
    /// When it was last opened; 0 when not known.
    #[n(8)]
    opened: i64,
}

#[derive(Default, Clone, State)]
struct AllApps {
    /// Whether Rapidkoi answered yet.
    read: bool,
    /// The person's apps, A–Z, as last read.
    apps: Vec<Item>,
    /// What's typed in the search box.
    query: String,
    /// Last used first, else A–Z.
    by_last: bool,
    /// The app whose details are open, by its index.
    details: Option<u32>,
}

impl AllApps {
    /// The apps shown: matching what's typed, in the chosen order.
    fn shown(&self) -> Vec<&Item> {
        let q = self.query.trim().to_lowercase();
        let mut out: Vec<&Item> = self.apps.iter().filter(|a| q.is_empty() || a.name.to_lowercase().contains(&q)).collect();
        if self.by_last {
            // Stable: apps opened at the same time keep their A–Z order.
            out.sort_by(|a, b| b.opened.cmp(&a.opened));
        }
        out
    }

    /// Whether another app has this one's name: then each shows where it's from.
    fn clashes(&self, a: &Item) -> bool {
        self.apps.iter().any(|b| b.at != a.at && b.name.to_lowercase() == a.name.to_lowercase())
    }

    fn ask(self, ask: Ask) -> Next<Self> {
        Next::state(AllApps { details: None, ..self }).ask(ask)
    }
}

impl App for AllApps {
    type Msg = Msg;

    fn handle(self, event: Event<Msg>) -> Next<Self> {
        match event {
            // Each time it's shown, or told the person's apps or pins changed.
            Event::Opened | Event::Resume | Event::Change { .. } => Next::state(self).ask(Ask::apps("apps")),
            Event::Data { msg, cbor } if msg == "apps" => match Apps::read(&cbor) {
                Some(read) => {
                    let apps = read
                        .items
                        .into_iter()
                        .enumerate()
                        .map(|(i, a)| Item {
                            at: i as u32,
                            name: a.name,
                            icon: a.icon,
                            from: a.from,
                            address: a.address,
                            publisher: a.publisher,
                            signed: a.signed,
                            pinned: a.pinned,
                            opened: a.opened,
                        })
                        .collect();
                    Next::state(AllApps { read: true, apps, ..self })
                }
                None => Next::state(self),
            },
            Event::Message { msg, value } => match msg {
                Msg::Typed => Next::state(AllApps { query: value.unwrap_or_default(), details: None, ..self }),
                Msg::Submit => match self.shown().first().map(|a| a.at) {
                    Some(i) => self.ask(Ask::open_app("open", i)),
                    None => Next::state(self),
                },
                Msg::Order(last) => Next::state(AllApps { by_last: last, ..self }),
                Msg::Details(i) => {
                    let which = if self.details == Some(i) { None } else { Some(i) };
                    Next::state(AllApps { details: which, ..self })
                }
                Msg::Open(i) => self.ask(Ask::open_app("open", i)),
                Msg::Pin(i) => self.ask(Ask::pin_app("pin", i)),
                Msg::Unpin(i) => self.ask(Ask::unpin_app("unpin", i)),
                Msg::Permissions(i) => self.ask(Ask::app_permissions("permissions", i)),
                Msg::Forget(i) => self.ask(Ask::forget_app("forget", i)),
            },
            _ => Next::state(self),
        }
    }

    fn view(&self) -> V {
        let head = row([text("All apps").title(), spacer(), order_switch(self.by_last)]).spacing(Space::Md);
        let shown = self.shown();
        // The first match is ringed while searching: Enter opens it.
        let first = if self.query.trim().is_empty() { None } else { shown.first().map(|a| a.at) };
        let grid = wrap(shown.iter().map(|a| self.tile(a, first == Some(a.at))))
            .style([Layout::equal(), Layout::per_line(8), Spacing::gap(Space::Px(8))]);
        let page = column([head, search(&self.query), grid])
            .spacing(Space::Lg)
            .padding(Space::Lg)
            // Rapidkoi never scrolls an app's page (PRP-046): a short window scrolls this.
            .style([Layout::scroll(), Layout::grow()]);
        let open = self.details.and_then(|i| self.apps.iter().find(|a| a.at == i));
        let screen = match open {
            // The details over the page, on a dimmed backdrop that closes them.
            Some(a) => layers([
                page,
                boxed()
                    .style([Layout::place(Place::Fill), Background::color(Color::rgba(0, 0, 0, 120))])
                    .label("Close")
                    .on_press(Msg::Details(a.at)),
                details(a).style([Sizing::width(360), Layout::place(Place::Center)]),
            ])
            .style([Layout::grow()]),
            None => page,
        };
        screen.screen_title("All apps")
    }
}

impl AllApps {
    /// One app: its icon and name (and where it's from, when names clash)
    /// opening it; a pin when it's on the dock.
    fn tile(&self, a: &Item, first: bool) -> V {
        let mut lines = vec![
            image(&a.icon).style([Sizing::width(56), Sizing::height(56), Layout::align(Align::Center)]),
            text(&a.name).style([Font::size(TextSize::Small), Layout::align(Align::Center), Text::truncate()]),
        ];
        if self.clashes(a) && !a.from.is_empty() {
            lines.push(
                text(&a.from).style([
                    Font::size(TextSize::Small),
                    Text::color(Color::system("text_dim")),
                    Layout::align(Align::Center),
                    Text::truncate(),
                ]),
            );
        }
        let mut look = vec![
            Sizing::height(132),
            Spacing::gap(Space::Sm),
            Spacing::padding_xy(Space::Px(18), Space::Px(8)),
            Border::corner(Corner::Px(12)),
            When(Look::Hover, [Background::color(lift(0.07))]),
        ];
        if first || self.details == Some(a.at) {
            look.push(Background::color(lift(0.07)));
        }
        if first {
            look.push(Border::all(2, Color::system("accent")));
        }
        let open = column(lines).style(look).label(&a.name).on_press(Msg::Open(a.at));
        let mut parts = vec![open];
        if a.pinned {
            parts.push(
                boxed()
                    .children([picture("pin.svg", 14, 14)])
                    .style([Layout::place(Place::TopStart), Spacing::padding(Space::Px(10))])
                    .label("On the dock"),
            );
        }
        // Its details open on a right-click once the contract has `on_context`
        // (PRP-068): no ⋯ on the tile (the owner, 2026-11-03).
        layers(parts)
    }
}

/// A–Z or Last used: two round segments in a round track, the chosen one lifted.
fn order_switch(by_last: bool) -> V {
    let segment = |word: &str, on: bool, msg: Msg| {
        let mut look = vec![Border::corner(Corner::Round), Spacing::padding_xy(Space::Px(6), Space::Px(14))];
        if on {
            look.push(Background::color(lift(0.16)));
        }
        boxed()
            .children([text(word).style([
                Font::size(TextSize::Small),
                Font::weight(if on { Weight::Bold } else { Weight::Normal }),
                Text::color(Color::system(if on { "text" } else { "text_dim" })),
            ])])
            .style(look)
            .role(Role::Tab)
            .selected(on)
            .on_press(msg)
    };
    boxed()
        .children([segment("A–Z", !by_last, Msg::Order(false)), segment("Last used", by_last, Msg::Order(true))])
        .style([
            Layout::row(),
            Spacing::gap(Space::Px(2)),
            Spacing::padding(Space::Px(3)),
            Border::corner(Corner::Round),
            Border::all(1, Color::system("border")),
            Background::color(lift(0.05)),
            Layout::align(Align::Center),
        ])
        .role(Role::TabList)
}

/// The search box: round, its magnifier inside at the start, the accent's ring while typing.
fn search(query: &str) -> V {
    let field = input(query)
        .placeholder("Search")
        .label("Search your apps")
        .on_change(Msg::Typed)
        .on_submit(Msg::Submit)
        .style([
            Border::corner(Corner::Round),
            Border::all(2, Color::system("border")),
            Spacing::padding_xy(Space::Px(10), Space::Px(44)),
            When(Look::Focus, [Border::all(2, Color::system("accent"))]),
        ]);
    layers([
        field,
        boxed()
            .children([picture("search.svg", 18, 18)])
            .style([Layout::place(Place::Start), Spacing::padding_xy(Space::Px(0), Space::Px(18))]),
    ])
}

/// An app's details, as a card: what it is and where it's from, then what can be done with it.
fn details(a: &Item) -> V {
    let row_of = |label: &str, value: V| {
        row([
            text(label).style([
                Font::size(TextSize::Small),
                Text::color(Color::system("text_dim")),
                Sizing::width(110),
            ]),
            value.style([Font::size(TextSize::Small)]),
        ])
        .spacing(Space::Md)
    };
    let publisher = if a.signed { format!("{}, signed", a.publisher) } else { String::from("Not signed") };
    let opened = if a.opened > 0 { text_of([rapidkoi::time(a.opened).relative()]) } else { text("Not yet") };
    let mut info = Vec::new();
    // Rapidkoi's own apps have no publisher or address to show.
    if !a.address.is_empty() {
        info.push(row_of("Publisher", text(&publisher)));
    }
    info.push(row_of("Last opened", opened));
    let mut who = vec![text(&a.name).heading()];
    if !a.address.is_empty() {
        who.push(text(&a.address).style([Font::size(TextSize::Small), Text::color(Color::system("text_dim"))]));
    }
    let top = row([
        image(&a.icon).style([Sizing::width(52), Sizing::height(52), Layout::align(Align::Center)]),
        column(who).spacing(Space::Xs),
    ])
    .spacing(Space::Md);
    let pin = if a.pinned { action("Unpin from dock", Msg::Unpin(a.at), "text") } else { action("Pin to dock", Msg::Pin(a.at), "text") };
    let actions = column([
        open_button(a.at),
        pin,
        action("Permissions", Msg::Permissions(a.at), "text"),
        boxed().style([Sizing::height(1), Background::color(Color::system("border"))]).role(Role::Separator),
        action("Forget this app…", Msg::Forget(a.at), "danger"),
    ])
    .spacing(Space::Xs);
    column([top, column(info).spacing(Space::Sm), actions])
        .spacing(Space::Md)
        .padding(Space::Lg)
        .style([
            Background::color(Color::system("surface")),
            Border::all(1, Color::system("border")),
            Border::corner(Corner::Px(12)),
        ])
}

/// The details' first action: Open, in the accent, across the card.
fn open_button(at: u32) -> V {
    boxed()
        .children([text("Open").style([
            Text::color(Color::system("on_accent")),
            Font::weight(Weight::Bold),
            Layout::align(Align::Center),
        ])])
        .style([
            Background::color(Color::system("accent")),
            Border::corner(Corner::Round),
            Spacing::padding_xy(Space::Px(10), Space::Px(16)),
        ])
        .on_press(Msg::Open(at))
}

/// One of the details' other actions: a round row, its words at the start in `ink`.
fn action(word: &str, msg: Msg, ink: &str) -> V {
    boxed()
        .children([text(word).style([Text::color(Color::system(ink))])])
        .style([
            Border::corner(Corner::Round),
            Spacing::padding_xy(Space::Px(10), Space::Px(12)),
            When(Look::Hover, [Background::color(lift(0.08))]),
        ])
        .on_press(msg)
}

/// A picture from the bundle, at its size.
fn picture(name: &str, w: u16, h: u16) -> V {
    debug_assert!(ASSETS.contains(&name), "{name} isn't packed");
    image(&format!("asset:{name}")).style([Sizing::width(w), Sizing::height(h)])
}

/// The page's colour lifted toward the text's.
fn lift(by: f64) -> Color {
    Color::system("background").mix(Color::system("text"), by)
}

rapidkoi::export!(AllApps);

#[cfg(test)]
mod tests;
