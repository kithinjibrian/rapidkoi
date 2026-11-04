//! Rapidkoi's All apps (PRP-067, DECISION-094): a built-in app, packed into
//! `./main` like Settings. It lists every app the person has opened, A–Z or
//! last used first, with a search box: typing filters by name and Enter
//! opens the first match. When two apps share a name, each also shows where
//! it's from. A right-click on an app opens its menu (PRP-071): Open, Pin or
//! Unpin, App info, Forget. App info and Forget are Rapidkoi's own screens
//! over All apps (PRP-072), where the person decides.
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
    column, image, input, item, layers, row, separator, spacer, text, wrap, App, Apps, Ask, Choice, Event,
    Message, Next, Space, State, View,
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
    Pin(u32),
    Unpin(u32),
    /// Rapidkoi's App info on this app (PRP-072).
    Info(u32),
    Forget(u32),
}

type V = View<Msg>;

/// Every picture the screen shows, each packed from `assets/` under its name.
const ASSETS: [&str; 1] = ["search.svg"];

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
        Next::state(self).ask(ask)
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
                Msg::Typed => Next::state(AllApps { query: value.unwrap_or_default(), ..self }),
                Msg::Submit => match self.shown().first().map(|a| a.at) {
                    Some(i) => self.ask(Ask::open_app("open", i)),
                    None => Next::state(self),
                },
                Msg::Order(last) => Next::state(AllApps { by_last: last, ..self }),
                Msg::Open(i) => self.ask(Ask::open_app("open", i)),
                Msg::Pin(i) => self.ask(Ask::pin_app("pin", i)),
                Msg::Unpin(i) => self.ask(Ask::unpin_app("unpin", i)),
                Msg::Info(i) => self.ask(Ask::app_info("info", i)),
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
            .style([Spacing::gap(Space::Px(4))]);
        column([head, search(&self.query), grid])
            .spacing(Space::Lg)
            .padding(Space::Lg)
            // Rapidkoi never scrolls an app's page (PRP-046): a short window scrolls this.
            .style([Layout::scroll(), Layout::grow()])
            .screen_title("All apps")
    }
}

impl AllApps {
    /// One app: its icon and name (and where it's from, when names clash),
    /// opening it.
    fn tile(&self, a: &Item, first: bool) -> V {
        // No mark for pinned apps: their menu says it (the owner, 2026-11-03).
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
            // A fixed width packed from the start: stretched columns spread
            // the apps apart (the owner, 2026-11-03).
            Sizing::width(104),
            Sizing::height(116),
            Spacing::gap(Space::Sm),
            Spacing::padding_xy(Space::Px(12), Space::Px(4)),
            Border::corner(Corner::Px(12)),
            When(Look::Hover, [Background::color(lift(0.07))]),
        ];
        if first {
            look.push(Background::color(lift(0.07)));
        }
        if first {
            look.push(Border::all(2, Color::system("accent")));
        }
        // A menu on a right-click (PRP-071): no ⋯ on the tile (the owner, 2026-11-03).
        column(lines).style(look).label(&a.name).on_press(Msg::Open(a.at)).menu(menu_of(a))
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

/// A tile's right-click menu (PRP-071; App info since PRP-072).
fn menu_of(a: &Item) -> Vec<Choice<Msg>> {
    let pin = if a.pinned { item("Unpin from dock", Msg::Unpin(a.at)) } else { item("Pin to dock", Msg::Pin(a.at)) };
    vec![
        item("Open", Msg::Open(a.at)),
        pin,
        item("App info", Msg::Info(a.at)),
        separator(),
        item("Forget this app…", Msg::Forget(a.at)),
    ]
}
